package grpcsrv

import (
	"context"
	"io"
	"io/ioutil"
	"net"
	"path/filepath"
	"testing"
	"time"

	"github.com/danjenson/motoko/motoko/internal/acctmgr"
	"github.com/danjenson/motoko/motoko/internal/grpcsrv/authorizer"
	mcred "github.com/danjenson/motoko/motoko/internal/grpcsrv/credentials"
	"github.com/danjenson/motoko/motoko/internal/grpcsrv/mocks"
	"github.com/danjenson/motoko/motoko/internal/pb"
	_ "github.com/go-sql-driver/mysql"
	"github.com/golang/protobuf/proto"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/test/bufconn"
)

func TestGRPCServer(t *testing.T) {
	a := assert.New(t)
	r := require.New(t)

	// resources
	creds := mcred.Credentials{
		Email:  "motoko.kusanagi@sector9.jp",
		APIKey: acctmgr.APIKey("12345"),
	}
	learnKey, evaluation, decisions := "abcde", "{}", "[]"
	ctx := context.Background()
	crtPath := filepath.Join("testdata", "keys", "test.crt")
	keyPath := filepath.Join("testdata", "keys", "test.key")
	tlsServer, err := credentials.NewServerTLSFromFile(crtPath, keyPath)
	r.Nil(err, "reading tls keys")
	tlsClient, err := credentials.NewClientTLSFromFile(crtPath, "")
	r.Nil(err, "reading client tls key")
	dataPath := filepath.Join("testdata", "data", "iris.csv")
	data, err := ioutil.ReadFile(dataPath)
	nDataBytes := uint64(len(data))
	r.Nil(err, "reading "+dataPath)
	metadataPath := filepath.Join("testdata", "data", "metadata.dat")
	metadataRaw, err := ioutil.ReadFile(metadataPath)
	r.Nil(err, "reading "+metadataPath)
	metadata := &pb.Metadata{}
	err = proto.Unmarshal(metadataRaw, metadata)
	r.Nil(err, "unmarshalling metadata")
	predictionsPath := filepath.Join("testdata", "data", "iris_predictions.json")
	predictionsRaw, err := ioutil.ReadFile(predictionsPath)
	r.Nil(err, "reading "+predictionsPath)
	predictions := string(predictionsRaw)
	am := &mocks.AccountManager{}
	tk := &mocks.TachikomaClient{}
	authorizer := authorizer.New(am)

	// server
	motoko := New(am, tk)
	bufnetBufferSize := int(1e6)
	lis := bufconn.Listen(bufnetBufferSize)
	server := grpc.NewServer(
		grpc.StreamInterceptor(authorizer.StreamInterceptor()),
		grpc.Creds(tlsServer),
	)
	pb.RegisterMotokoServer(server, motoko)
	go func() {
		err := server.Serve(lis)
		a.Nil(err, "starting server")
	}()
	defer server.GracefulStop()

	// client
	bufnetDialer := func(string, time.Duration) (net.Conn, error) {
		return lis.Dial()
	}
	conn, err := grpc.DialContext(
		ctx,
		"bufnet",
		grpc.WithDialer(bufnetDialer),
		grpc.WithPerRPCCredentials(creds),
		grpc.WithTransportCredentials(tlsClient),
	)
	r.Nil(err, "creating connection")
	defer conn.Close()
	client := pb.NewMotokoClient(conn)

	// NOTE: can't run subtests in parallel because they all depend on
	// common server and client which will be shutdown / closed by the time
	// parallel subtests run

	t.Run("infer", func(t *testing.T) {
		tkInfer := &mocks.Tachikoma_InferClient{}
		inferReqParams := &pb.InferRequest{
			Value: &pb.InferRequest_Parameters_{
				Parameters: &pb.InferRequest_Parameters{
					NumericErrorThreshold: 0.05,
					NMaxCategories:        25,
				},
			},
		}
		inferReqData := &pb.InferRequest{
			Value: &pb.InferRequest_Data{Data: data},
		}
		inferRes := &pb.InferResponse{
			Metadata: metadata,
		}
		tk.On("Infer", mock.Anything).Return(tkInfer, nil).Once()
		am.On("IsActiveAPIKey", creds.Email, creds.APIKey).Return(true, nil).Once()
		tkInfer.On("Send", mock.Anything).Return(nil).Twice()
		tkInfer.On("CloseAndRecv").Return(inferRes, nil).Once()
		mInfer, err := client.Infer(ctx)
		a.Nil(err, "creating motoko infer stream")
		err = mInfer.Send(inferReqParams)
		a.Nil(err, "sending infer params")
		err = mInfer.Send(inferReqData)
		a.Nil(err, "sending infer data")
		inferResRet, err := mInfer.CloseAndRecv()
		a.Nil(err, "receiving infer response")
		eq := proto.Equal(inferResRet, inferRes)
		a.True(eq, "responses not equivalent")
		tk.AssertExpectations(t)
		tkInfer.AssertExpectations(t)
	})

	t.Run("learn", func(t *testing.T) {
		tkLearn := &mocks.Tachikoma_LearnClient{}
		learnReqMetadata := &pb.LearnRequest{
			Value: &pb.LearnRequest_Metadata{
				Metadata: metadata,
			},
		}
		learnReqData := &pb.LearnRequest{
			Value: &pb.LearnRequest_Data{
				Data: data,
			},
		}
		learnRes := &pb.LearnResponse{
			LearnKey:   learnKey,
			Evaluation: evaluation,
			Decisions:  decisions,
		}
		tk.On("Learn", mock.Anything).Return(tkLearn, nil).Once()
		am.On("IsActiveAPIKey", creds.Email, creds.APIKey).Return(true, nil).Once()
		tkLearn.On("Send", mock.Anything).Return(nil).Twice()
		tkLearn.On("CloseAndRecv").Return(learnRes, nil).Once()
		am.On(
			"RegisterAPICall",
			creds.APIKey,
			learnKey,
			acctmgr.LEARN,
			nDataBytes,
		).Return(nil)
		mLearn, err := client.Learn(ctx)
		a.Nil(err, "creating learn stream")
		err = mLearn.Send(learnReqMetadata)
		a.Nil(err, "sending learn metadata")
		err = mLearn.Send(learnReqData)
		a.Nil(err, "sending learn data")
		learnResRet, err := mLearn.CloseAndRecv()
		a.Nil(err, "receiving learn response")
		eq := proto.Equal(learnResRet, learnRes)
		a.True(eq, "responses not equivalent")
		tk.AssertExpectations(t)
		tkLearn.AssertExpectations(t)
	})

	t.Run("predict", func(t *testing.T) {
		tkPredict := &mocks.Tachikoma_PredictClient{}
		predictReqLearnKey := &pb.PredictRequest{
			Value: &pb.PredictRequest_LearnKey{
				LearnKey: learnKey,
			},
		}
		predictReqData := &pb.PredictRequest{
			Value: &pb.PredictRequest_Data{
				Data: data,
			},
		}
		predictResPredictions := &pb.PredictResponse{
			Value: &pb.PredictResponse_Predictions{
				Predictions: predictions,
			},
		}
		predictResDecisions := &pb.PredictResponse{
			Value: &pb.PredictResponse_Decisions{
				Decisions: decisions,
			},
		}
		predictResEmpty := &pb.PredictResponse{}
		tk.On("Predict", mock.Anything).Return(tkPredict, nil).Once()
		am.On("IsActiveAPIKey", creds.Email, creds.APIKey).Return(true, nil).Once()
		tkPredict.On("Send", mock.Anything).Return(nil).Twice()
		tkPredict.On("CloseSend").Return(nil).Once()
		tkPredict.On("Recv").Return(predictResPredictions, nil).Once()
		tkPredict.On("Recv").Return(predictResDecisions, nil).Once()
		tkPredict.On("Recv").Return(predictResEmpty, io.EOF).Once()
		am.On(
			"RegisterAPICall",
			creds.APIKey,
			learnKey,
			acctmgr.PREDICT,
			nDataBytes,
		).Return(nil)
		mPredict, err := client.Predict(ctx)
		a.Nil(err, "creating motoko predict stream")
		err = mPredict.Send(predictReqLearnKey)
		a.Nil(err, "sending predict learn_key")
		err = mPredict.Send(predictReqData)
		a.Nil(err, "sending predict data")
		err = mPredict.CloseSend()
		a.Nil(err, "closing send on predict stream")
		predictResPredictionsRet, err := mPredict.Recv()
		a.Nil(err, "receiving predict predictions response")
		eq := proto.Equal(predictResPredictionsRet, predictResPredictions)
		a.True(eq, "predictions response not equivalent")
		predictResDecisionsRet, err := mPredict.Recv()
		a.Nil(err, "receiving predict decisions response")
		eq = proto.Equal(predictResDecisionsRet, predictResDecisions)
		a.True(eq, "decisions response not equivalent")
		tk.AssertExpectations(t)
		tkPredict.AssertExpectations(t)
	})

	t.Run("web infer", func(t *testing.T) {
		tkInfer := &mocks.Tachikoma_InferClient{}
		inferReqParams := &pb.InferRequest{
			Value: &pb.InferRequest_Parameters_{
				Parameters: &pb.InferRequest_Parameters{
					NumericErrorThreshold: 0.05,
					NMaxCategories:        25,
				},
			},
		}
		inferReqData := &pb.InferRequest{
			Value: &pb.InferRequest_Data{Data: data},
		}
		inferRes := &pb.InferResponse{
			Metadata: metadata,
		}
		tk.On("Infer", mock.Anything).Return(tkInfer, nil)
		am.On("IsActiveAPIKey", creds.Email, creds.APIKey).Return(true, nil)
		tkInfer.On("Send", mock.Anything).Return(nil).Twice()
		tkInfer.On("CloseAndRecv").Return(inferRes, nil)
		mWebInfer, err := client.WebInfer(ctx)
		a.Nil(err, "creating motoko infer stream")
		err = mWebInfer.Send(inferReqParams)
		a.Nil(err, "sending infer params")
		err = mWebInfer.Send(inferReqData)
		a.Nil(err, "sending infer data")
		err = mWebInfer.CloseSend()
		a.Nil(err, "closing send")
		inferResRet, err := mWebInfer.Recv()
		a.Nil(err, "receiving infer response")
		eq := proto.Equal(inferResRet, inferRes)
		a.True(eq, "responses not equivalent")
		_, err = mWebInfer.Recv()
		a.Error(err, io.EOF)
		tk.AssertExpectations(t)
		tkInfer.AssertExpectations(t)
	})

	t.Run("web learn", func(t *testing.T) {
		tkLearn := &mocks.Tachikoma_LearnClient{}
		learnReqMetadata := &pb.LearnRequest{
			Value: &pb.LearnRequest_Metadata{
				Metadata: metadata,
			},
		}
		learnReqData := &pb.LearnRequest{
			Value: &pb.LearnRequest_Data{
				Data: data,
			},
		}
		learnRes := &pb.LearnResponse{
			LearnKey:   learnKey,
			Evaluation: evaluation,
			Decisions:  decisions,
		}
		tk.On("Learn", mock.Anything).Return(tkLearn, nil).Once()
		am.On("IsActiveAPIKey", creds.Email, creds.APIKey).Return(true, nil).Once()
		tkLearn.On("Send", mock.Anything).Return(nil).Twice()
		tkLearn.On("CloseAndRecv").Return(learnRes, nil).Once()
		am.On(
			"RegisterAPICall",
			creds.APIKey,
			learnKey,
			acctmgr.LEARN,
			nDataBytes,
		).Return(nil)
		mWebLearn, err := client.WebLearn(ctx)
		a.Nil(err, "creating web learn stream")
		err = mWebLearn.Send(learnReqMetadata)
		a.Nil(err, "sending learn metadata")
		err = mWebLearn.Send(learnReqData)
		a.Nil(err, "sending learn data")
		err = mWebLearn.CloseSend()
		a.Nil(err, "closing send")
		learnResRet, err := mWebLearn.Recv()
		a.Nil(err, "receiving learn response")
		eq := proto.Equal(learnResRet, learnRes)
		a.True(eq, "responses not equivalent")
		_, err = mWebLearn.Recv()
		a.Error(err, io.EOF)
		tk.AssertExpectations(t)
		tkLearn.AssertExpectations(t)
	})
}
