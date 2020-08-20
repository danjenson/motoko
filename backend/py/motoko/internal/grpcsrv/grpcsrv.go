package grpcsrv

import (
	"context"
	"io"

	"github.com/danjenson/motoko/motoko/internal/acctmgr"
	"github.com/danjenson/motoko/motoko/internal/pb"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/metadata"
	"google.golang.org/grpc/status"
)

type server struct {
	am acctmgr.AccountManager
	tk pb.TachikomaClient
}

func New(
	am acctmgr.AccountManager,
	tk pb.TachikomaClient,
) pb.MotokoServer {
	return &server{am, tk}
}

func (s *server) Infer(in pb.Motoko_InferServer) error {
	ctx := in.Context()
	out, err := s.tk.Infer(ctx)
	if err != nil {
		return err
	}
	for {
		req, err := in.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			return err
		}
		if err = out.Send(req); err != nil {
			return err
		}
	}
	res, err := out.CloseAndRecv()
	if err != nil {
		return err
	}
	return in.SendAndClose(res)
}

func (s *server) Learn(in pb.Motoko_LearnServer) error {
	ctx := in.Context()
	out, err := s.tk.Learn(ctx)
	if err != nil {
		return err
	}
	apiKey, err := getAPIKey(ctx)
	if err != nil {
		return err
	}
	var nBytes uint64
	for {
		req, err := in.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			return err
		}
		if err = out.Send(req); err != nil {
			return err
		}
		nBytes += uint64(len(req.GetData()))
	}
	res, err := out.CloseAndRecv()
	if err != nil {
		return err
	}
	learnKey := res.GetLearnKey()
	if err = in.SendAndClose(res); err != nil {
		return err
	}
	return s.am.RegisterAPICall(apiKey, learnKey, acctmgr.LEARN, nBytes)
}

func getAPIKey(ctx context.Context) (acctmgr.APIKey, error) {
	md, ok := metadata.FromIncomingContext(ctx)
	apiKey := md["api_key"]
	if !ok || len(apiKey) < 1 {
		return "", status.Errorf(codes.InvalidArgument, "missing metadata")
	}
	return acctmgr.APIKey(apiKey[0]), nil
}

func (s *server) Predict(in pb.Motoko_PredictServer) error {
	ctx := in.Context()
	out, err := s.tk.Predict(ctx)
	if err != nil {
		return err
	}
	apiKey, err := getAPIKey(ctx)
	if err != nil {
		return err
	}
	var nBytes uint64
	var learnKey string
	for {
		req, err := in.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			return err
		}
		switch x := req.Value.(type) {
		case *pb.PredictRequest_LearnKey:
			learnKey = x.LearnKey
		}
		if err = out.Send(req); err != nil {
			return err
		}
		nBytes += uint64(len(req.GetData()))
	}
	err = out.CloseSend()
	if err != nil {
		return err
	}
	for {
		res, err := out.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			return err
		}
		if err = in.Send(res); err != nil {
			return err
		}
	}
	return s.am.RegisterAPICall(apiKey, learnKey, acctmgr.PREDICT, nBytes)
}

func (s *server) WebInfer(in pb.Motoko_WebInferServer) error {
	ctx := in.Context()
	out, err := s.tk.Infer(ctx)
	if err != nil {
		return err
	}
	for {
		req, err := in.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			return err
		}
		if err = out.Send(req); err != nil {
			return err
		}
	}
	res, err := out.CloseAndRecv()
	if err != nil {
		return err
	}
	return in.Send(res)
}

func (s *server) WebLearn(in pb.Motoko_WebLearnServer) error {
	ctx := in.Context()
	out, err := s.tk.Learn(ctx)
	if err != nil {
		return err
	}
	apiKey, err := getAPIKey(ctx)
	if err != nil {
		return err
	}
	var nBytes uint64
	for {
		req, err := in.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			return err
		}
		if err = out.Send(req); err != nil {
			return err
		}
		nBytes += uint64(len(req.GetData()))
	}
	res, err := out.CloseAndRecv()
	if err != nil {
		return err
	}
	learnKey := res.GetLearnKey()
	if err = in.Send(res); err != nil {
		return err
	}
	return s.am.RegisterAPICall(apiKey, learnKey, acctmgr.LEARN, nBytes)
}
