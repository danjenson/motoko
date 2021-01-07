// Code generated by protoc-gen-go. DO NOT EDIT.
// source: types.proto

package pb

import (
	fmt "fmt"
	proto "github.com/golang/protobuf/proto"
	math "math"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.ProtoPackageIsVersion3 // please upgrade the proto package

type Attribute struct {
	Name                 string         `protobuf:"bytes,1,opt,name=name,proto3" json:"name,omitempty"`
	BehaviorType         BehaviorType   `protobuf:"varint,2,opt,name=behavior_type,json=behaviorType,proto3,enum=pb.BehaviorType" json:"behavior_type,omitempty"`
	DataType             DataType       `protobuf:"varint,3,opt,name=data_type,json=dataType,proto3,enum=pb.DataType" json:"data_type,omitempty"`
	Replacements         []*Replacement `protobuf:"bytes,4,rep,name=replacements,proto3" json:"replacements,omitempty"`
	XXX_NoUnkeyedLiteral struct{}       `json:"-"`
	XXX_unrecognized     []byte         `json:"-"`
	XXX_sizecache        int32          `json:"-"`
}

func (m *Attribute) Reset()         { *m = Attribute{} }
func (m *Attribute) String() string { return proto.CompactTextString(m) }
func (*Attribute) ProtoMessage()    {}
func (*Attribute) Descriptor() ([]byte, []int) {
	return fileDescriptor_d938547f84707355, []int{0}
}

func (m *Attribute) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_Attribute.Unmarshal(m, b)
}
func (m *Attribute) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_Attribute.Marshal(b, m, deterministic)
}
func (m *Attribute) XXX_Merge(src proto.Message) {
	xxx_messageInfo_Attribute.Merge(m, src)
}
func (m *Attribute) XXX_Size() int {
	return xxx_messageInfo_Attribute.Size(m)
}
func (m *Attribute) XXX_DiscardUnknown() {
	xxx_messageInfo_Attribute.DiscardUnknown(m)
}

var xxx_messageInfo_Attribute proto.InternalMessageInfo

func (m *Attribute) GetName() string {
	if m != nil {
		return m.Name
	}
	return ""
}

func (m *Attribute) GetBehaviorType() BehaviorType {
	if m != nil {
		return m.BehaviorType
	}
	return BehaviorType_CONTINUOUS
}

func (m *Attribute) GetDataType() DataType {
	if m != nil {
		return m.DataType
	}
	return DataType_BOOLEAN
}

func (m *Attribute) GetReplacements() []*Replacement {
	if m != nil {
		return m.Replacements
	}
	return nil
}

type InferRequest struct {
	// Types that are valid to be assigned to Value:
	//	*InferRequest_Parameters_
	//	*InferRequest_Data
	Value                isInferRequest_Value `protobuf_oneof:"value"`
	XXX_NoUnkeyedLiteral struct{}             `json:"-"`
	XXX_unrecognized     []byte               `json:"-"`
	XXX_sizecache        int32                `json:"-"`
}

func (m *InferRequest) Reset()         { *m = InferRequest{} }
func (m *InferRequest) String() string { return proto.CompactTextString(m) }
func (*InferRequest) ProtoMessage()    {}
func (*InferRequest) Descriptor() ([]byte, []int) {
	return fileDescriptor_d938547f84707355, []int{1}
}

func (m *InferRequest) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_InferRequest.Unmarshal(m, b)
}
func (m *InferRequest) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_InferRequest.Marshal(b, m, deterministic)
}
func (m *InferRequest) XXX_Merge(src proto.Message) {
	xxx_messageInfo_InferRequest.Merge(m, src)
}
func (m *InferRequest) XXX_Size() int {
	return xxx_messageInfo_InferRequest.Size(m)
}
func (m *InferRequest) XXX_DiscardUnknown() {
	xxx_messageInfo_InferRequest.DiscardUnknown(m)
}

var xxx_messageInfo_InferRequest proto.InternalMessageInfo

type isInferRequest_Value interface {
	isInferRequest_Value()
}

type InferRequest_Parameters_ struct {
	Parameters *InferRequest_Parameters `protobuf:"bytes,1,opt,name=parameters,proto3,oneof"`
}

type InferRequest_Data struct {
	Data []byte `protobuf:"bytes,2,opt,name=data,proto3,oneof"`
}

func (*InferRequest_Parameters_) isInferRequest_Value() {}

func (*InferRequest_Data) isInferRequest_Value() {}

func (m *InferRequest) GetValue() isInferRequest_Value {
	if m != nil {
		return m.Value
	}
	return nil
}

func (m *InferRequest) GetParameters() *InferRequest_Parameters {
	if x, ok := m.GetValue().(*InferRequest_Parameters_); ok {
		return x.Parameters
	}
	return nil
}

func (m *InferRequest) GetData() []byte {
	if x, ok := m.GetValue().(*InferRequest_Data); ok {
		return x.Data
	}
	return nil
}

// XXX_OneofWrappers is for the internal use of the proto package.
func (*InferRequest) XXX_OneofWrappers() []interface{} {
	return []interface{}{
		(*InferRequest_Parameters_)(nil),
		(*InferRequest_Data)(nil),
	}
}

type InferRequest_Parameters struct {
	NumericErrorThreshold float32  `protobuf:"fixed32,1,opt,name=numeric_error_threshold,json=numericErrorThreshold,proto3" json:"numeric_error_threshold,omitempty"`
	NMaxCategories        int32    `protobuf:"varint,2,opt,name=n_max_categories,json=nMaxCategories,proto3" json:"n_max_categories,omitempty"`
	XXX_NoUnkeyedLiteral  struct{} `json:"-"`
	XXX_unrecognized      []byte   `json:"-"`
	XXX_sizecache         int32    `json:"-"`
}

func (m *InferRequest_Parameters) Reset()         { *m = InferRequest_Parameters{} }
func (m *InferRequest_Parameters) String() string { return proto.CompactTextString(m) }
func (*InferRequest_Parameters) ProtoMessage()    {}
func (*InferRequest_Parameters) Descriptor() ([]byte, []int) {
	return fileDescriptor_d938547f84707355, []int{1, 0}
}

func (m *InferRequest_Parameters) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_InferRequest_Parameters.Unmarshal(m, b)
}
func (m *InferRequest_Parameters) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_InferRequest_Parameters.Marshal(b, m, deterministic)
}
func (m *InferRequest_Parameters) XXX_Merge(src proto.Message) {
	xxx_messageInfo_InferRequest_Parameters.Merge(m, src)
}
func (m *InferRequest_Parameters) XXX_Size() int {
	return xxx_messageInfo_InferRequest_Parameters.Size(m)
}
func (m *InferRequest_Parameters) XXX_DiscardUnknown() {
	xxx_messageInfo_InferRequest_Parameters.DiscardUnknown(m)
}

var xxx_messageInfo_InferRequest_Parameters proto.InternalMessageInfo

func (m *InferRequest_Parameters) GetNumericErrorThreshold() float32 {
	if m != nil {
		return m.NumericErrorThreshold
	}
	return 0
}

func (m *InferRequest_Parameters) GetNMaxCategories() int32 {
	if m != nil {
		return m.NMaxCategories
	}
	return 0
}

type InferResponse struct {
	Metadata             *Metadata `protobuf:"bytes,1,opt,name=metadata,proto3" json:"metadata,omitempty"`
	XXX_NoUnkeyedLiteral struct{}  `json:"-"`
	XXX_unrecognized     []byte    `json:"-"`
	XXX_sizecache        int32     `json:"-"`
}

func (m *InferResponse) Reset()         { *m = InferResponse{} }
func (m *InferResponse) String() string { return proto.CompactTextString(m) }
func (*InferResponse) ProtoMessage()    {}
func (*InferResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_d938547f84707355, []int{2}
}

func (m *InferResponse) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_InferResponse.Unmarshal(m, b)
}
func (m *InferResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_InferResponse.Marshal(b, m, deterministic)
}
func (m *InferResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_InferResponse.Merge(m, src)
}
func (m *InferResponse) XXX_Size() int {
	return xxx_messageInfo_InferResponse.Size(m)
}
func (m *InferResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_InferResponse.DiscardUnknown(m)
}

var xxx_messageInfo_InferResponse proto.InternalMessageInfo

func (m *InferResponse) GetMetadata() *Metadata {
	if m != nil {
		return m.Metadata
	}
	return nil
}

type LearnRequest struct {
	// Types that are valid to be assigned to Value:
	//	*LearnRequest_Metadata
	//	*LearnRequest_Data
	Value                isLearnRequest_Value `protobuf_oneof:"value"`
	XXX_NoUnkeyedLiteral struct{}             `json:"-"`
	XXX_unrecognized     []byte               `json:"-"`
	XXX_sizecache        int32                `json:"-"`
}

func (m *LearnRequest) Reset()         { *m = LearnRequest{} }
func (m *LearnRequest) String() string { return proto.CompactTextString(m) }
func (*LearnRequest) ProtoMessage()    {}
func (*LearnRequest) Descriptor() ([]byte, []int) {
	return fileDescriptor_d938547f84707355, []int{3}
}

func (m *LearnRequest) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_LearnRequest.Unmarshal(m, b)
}
func (m *LearnRequest) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_LearnRequest.Marshal(b, m, deterministic)
}
func (m *LearnRequest) XXX_Merge(src proto.Message) {
	xxx_messageInfo_LearnRequest.Merge(m, src)
}
func (m *LearnRequest) XXX_Size() int {
	return xxx_messageInfo_LearnRequest.Size(m)
}
func (m *LearnRequest) XXX_DiscardUnknown() {
	xxx_messageInfo_LearnRequest.DiscardUnknown(m)
}

var xxx_messageInfo_LearnRequest proto.InternalMessageInfo

type isLearnRequest_Value interface {
	isLearnRequest_Value()
}

type LearnRequest_Metadata struct {
	Metadata *Metadata `protobuf:"bytes,1,opt,name=metadata,proto3,oneof"`
}

type LearnRequest_Data struct {
	Data []byte `protobuf:"bytes,2,opt,name=data,proto3,oneof"`
}

func (*LearnRequest_Metadata) isLearnRequest_Value() {}

func (*LearnRequest_Data) isLearnRequest_Value() {}

func (m *LearnRequest) GetValue() isLearnRequest_Value {
	if m != nil {
		return m.Value
	}
	return nil
}

func (m *LearnRequest) GetMetadata() *Metadata {
	if x, ok := m.GetValue().(*LearnRequest_Metadata); ok {
		return x.Metadata
	}
	return nil
}

func (m *LearnRequest) GetData() []byte {
	if x, ok := m.GetValue().(*LearnRequest_Data); ok {
		return x.Data
	}
	return nil
}

// XXX_OneofWrappers is for the internal use of the proto package.
func (*LearnRequest) XXX_OneofWrappers() []interface{} {
	return []interface{}{
		(*LearnRequest_Metadata)(nil),
		(*LearnRequest_Data)(nil),
	}
}

type LearnResponse struct {
	LearnKey             string   `protobuf:"bytes,1,opt,name=learn_key,json=learnKey,proto3" json:"learn_key,omitempty"`
	Evaluation           string   `protobuf:"bytes,2,opt,name=evaluation,proto3" json:"evaluation,omitempty"`
	Decisions            string   `protobuf:"bytes,3,opt,name=decisions,proto3" json:"decisions,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *LearnResponse) Reset()         { *m = LearnResponse{} }
func (m *LearnResponse) String() string { return proto.CompactTextString(m) }
func (*LearnResponse) ProtoMessage()    {}
func (*LearnResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_d938547f84707355, []int{4}
}

func (m *LearnResponse) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_LearnResponse.Unmarshal(m, b)
}
func (m *LearnResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_LearnResponse.Marshal(b, m, deterministic)
}
func (m *LearnResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_LearnResponse.Merge(m, src)
}
func (m *LearnResponse) XXX_Size() int {
	return xxx_messageInfo_LearnResponse.Size(m)
}
func (m *LearnResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_LearnResponse.DiscardUnknown(m)
}

var xxx_messageInfo_LearnResponse proto.InternalMessageInfo

func (m *LearnResponse) GetLearnKey() string {
	if m != nil {
		return m.LearnKey
	}
	return ""
}

func (m *LearnResponse) GetEvaluation() string {
	if m != nil {
		return m.Evaluation
	}
	return ""
}

func (m *LearnResponse) GetDecisions() string {
	if m != nil {
		return m.Decisions
	}
	return ""
}

type Metadata struct {
	HasTarget  bool   `protobuf:"varint,1,opt,name=has_target,json=hasTarget,proto3" json:"has_target,omitempty"`
	TargetName string `protobuf:"bytes,2,opt,name=target_name,json=targetName,proto3" json:"target_name,omitempty"`
	// list instead of Map because maps do not allow repeated fields
	// and attributes have repeated Replacements
	Attributes           []*Attribute `protobuf:"bytes,3,rep,name=attributes,proto3" json:"attributes,omitempty"`
	XXX_NoUnkeyedLiteral struct{}     `json:"-"`
	XXX_unrecognized     []byte       `json:"-"`
	XXX_sizecache        int32        `json:"-"`
}

func (m *Metadata) Reset()         { *m = Metadata{} }
func (m *Metadata) String() string { return proto.CompactTextString(m) }
func (*Metadata) ProtoMessage()    {}
func (*Metadata) Descriptor() ([]byte, []int) {
	return fileDescriptor_d938547f84707355, []int{5}
}

func (m *Metadata) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_Metadata.Unmarshal(m, b)
}
func (m *Metadata) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_Metadata.Marshal(b, m, deterministic)
}
func (m *Metadata) XXX_Merge(src proto.Message) {
	xxx_messageInfo_Metadata.Merge(m, src)
}
func (m *Metadata) XXX_Size() int {
	return xxx_messageInfo_Metadata.Size(m)
}
func (m *Metadata) XXX_DiscardUnknown() {
	xxx_messageInfo_Metadata.DiscardUnknown(m)
}

var xxx_messageInfo_Metadata proto.InternalMessageInfo

func (m *Metadata) GetHasTarget() bool {
	if m != nil {
		return m.HasTarget
	}
	return false
}

func (m *Metadata) GetTargetName() string {
	if m != nil {
		return m.TargetName
	}
	return ""
}

func (m *Metadata) GetAttributes() []*Attribute {
	if m != nil {
		return m.Attributes
	}
	return nil
}

type PredictRequest struct {
	// Types that are valid to be assigned to Value:
	//	*PredictRequest_LearnKey
	//	*PredictRequest_Data
	Value                isPredictRequest_Value `protobuf_oneof:"value"`
	XXX_NoUnkeyedLiteral struct{}               `json:"-"`
	XXX_unrecognized     []byte                 `json:"-"`
	XXX_sizecache        int32                  `json:"-"`
}

func (m *PredictRequest) Reset()         { *m = PredictRequest{} }
func (m *PredictRequest) String() string { return proto.CompactTextString(m) }
func (*PredictRequest) ProtoMessage()    {}
func (*PredictRequest) Descriptor() ([]byte, []int) {
	return fileDescriptor_d938547f84707355, []int{6}
}

func (m *PredictRequest) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_PredictRequest.Unmarshal(m, b)
}
func (m *PredictRequest) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_PredictRequest.Marshal(b, m, deterministic)
}
func (m *PredictRequest) XXX_Merge(src proto.Message) {
	xxx_messageInfo_PredictRequest.Merge(m, src)
}
func (m *PredictRequest) XXX_Size() int {
	return xxx_messageInfo_PredictRequest.Size(m)
}
func (m *PredictRequest) XXX_DiscardUnknown() {
	xxx_messageInfo_PredictRequest.DiscardUnknown(m)
}

var xxx_messageInfo_PredictRequest proto.InternalMessageInfo

type isPredictRequest_Value interface {
	isPredictRequest_Value()
}

type PredictRequest_LearnKey struct {
	LearnKey string `protobuf:"bytes,1,opt,name=learn_key,json=learnKey,proto3,oneof"`
}

type PredictRequest_Data struct {
	Data []byte `protobuf:"bytes,2,opt,name=data,proto3,oneof"`
}

func (*PredictRequest_LearnKey) isPredictRequest_Value() {}

func (*PredictRequest_Data) isPredictRequest_Value() {}

func (m *PredictRequest) GetValue() isPredictRequest_Value {
	if m != nil {
		return m.Value
	}
	return nil
}

func (m *PredictRequest) GetLearnKey() string {
	if x, ok := m.GetValue().(*PredictRequest_LearnKey); ok {
		return x.LearnKey
	}
	return ""
}

func (m *PredictRequest) GetData() []byte {
	if x, ok := m.GetValue().(*PredictRequest_Data); ok {
		return x.Data
	}
	return nil
}

// XXX_OneofWrappers is for the internal use of the proto package.
func (*PredictRequest) XXX_OneofWrappers() []interface{} {
	return []interface{}{
		(*PredictRequest_LearnKey)(nil),
		(*PredictRequest_Data)(nil),
	}
}

type PredictResponse struct {
	// Types that are valid to be assigned to Value:
	//	*PredictResponse_Predictions
	//	*PredictResponse_Decisions
	Value                isPredictResponse_Value `protobuf_oneof:"value"`
	XXX_NoUnkeyedLiteral struct{}                `json:"-"`
	XXX_unrecognized     []byte                  `json:"-"`
	XXX_sizecache        int32                   `json:"-"`
}

func (m *PredictResponse) Reset()         { *m = PredictResponse{} }
func (m *PredictResponse) String() string { return proto.CompactTextString(m) }
func (*PredictResponse) ProtoMessage()    {}
func (*PredictResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_d938547f84707355, []int{7}
}

func (m *PredictResponse) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_PredictResponse.Unmarshal(m, b)
}
func (m *PredictResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_PredictResponse.Marshal(b, m, deterministic)
}
func (m *PredictResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_PredictResponse.Merge(m, src)
}
func (m *PredictResponse) XXX_Size() int {
	return xxx_messageInfo_PredictResponse.Size(m)
}
func (m *PredictResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_PredictResponse.DiscardUnknown(m)
}

var xxx_messageInfo_PredictResponse proto.InternalMessageInfo

type isPredictResponse_Value interface {
	isPredictResponse_Value()
}

type PredictResponse_Predictions struct {
	Predictions string `protobuf:"bytes,1,opt,name=predictions,proto3,oneof"`
}

type PredictResponse_Decisions struct {
	Decisions string `protobuf:"bytes,2,opt,name=decisions,proto3,oneof"`
}

func (*PredictResponse_Predictions) isPredictResponse_Value() {}

func (*PredictResponse_Decisions) isPredictResponse_Value() {}

func (m *PredictResponse) GetValue() isPredictResponse_Value {
	if m != nil {
		return m.Value
	}
	return nil
}

func (m *PredictResponse) GetPredictions() string {
	if x, ok := m.GetValue().(*PredictResponse_Predictions); ok {
		return x.Predictions
	}
	return ""
}

func (m *PredictResponse) GetDecisions() string {
	if x, ok := m.GetValue().(*PredictResponse_Decisions); ok {
		return x.Decisions
	}
	return ""
}

// XXX_OneofWrappers is for the internal use of the proto package.
func (*PredictResponse) XXX_OneofWrappers() []interface{} {
	return []interface{}{
		(*PredictResponse_Predictions)(nil),
		(*PredictResponse_Decisions)(nil),
	}
}

type Replacement struct {
	From                 *Replacement_From `protobuf:"bytes,1,opt,name=from,proto3" json:"from,omitempty"`
	To                   *Replacement_To   `protobuf:"bytes,2,opt,name=to,proto3" json:"to,omitempty"`
	XXX_NoUnkeyedLiteral struct{}          `json:"-"`
	XXX_unrecognized     []byte            `json:"-"`
	XXX_sizecache        int32             `json:"-"`
}

func (m *Replacement) Reset()         { *m = Replacement{} }
func (m *Replacement) String() string { return proto.CompactTextString(m) }
func (*Replacement) ProtoMessage()    {}
func (*Replacement) Descriptor() ([]byte, []int) {
	return fileDescriptor_d938547f84707355, []int{8}
}

func (m *Replacement) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_Replacement.Unmarshal(m, b)
}
func (m *Replacement) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_Replacement.Marshal(b, m, deterministic)
}
func (m *Replacement) XXX_Merge(src proto.Message) {
	xxx_messageInfo_Replacement.Merge(m, src)
}
func (m *Replacement) XXX_Size() int {
	return xxx_messageInfo_Replacement.Size(m)
}
func (m *Replacement) XXX_DiscardUnknown() {
	xxx_messageInfo_Replacement.DiscardUnknown(m)
}

var xxx_messageInfo_Replacement proto.InternalMessageInfo

func (m *Replacement) GetFrom() *Replacement_From {
	if m != nil {
		return m.From
	}
	return nil
}

func (m *Replacement) GetTo() *Replacement_To {
	if m != nil {
		return m.To
	}
	return nil
}

type Replacement_From struct {
	// Types that are valid to be assigned to Value:
	//	*Replacement_From_Missing
	//	*Replacement_From_Datum
	Value                isReplacement_From_Value `protobuf_oneof:"value"`
	XXX_NoUnkeyedLiteral struct{}                 `json:"-"`
	XXX_unrecognized     []byte                   `json:"-"`
	XXX_sizecache        int32                    `json:"-"`
}

func (m *Replacement_From) Reset()         { *m = Replacement_From{} }
func (m *Replacement_From) String() string { return proto.CompactTextString(m) }
func (*Replacement_From) ProtoMessage()    {}
func (*Replacement_From) Descriptor() ([]byte, []int) {
	return fileDescriptor_d938547f84707355, []int{8, 0}
}

func (m *Replacement_From) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_Replacement_From.Unmarshal(m, b)
}
func (m *Replacement_From) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_Replacement_From.Marshal(b, m, deterministic)
}
func (m *Replacement_From) XXX_Merge(src proto.Message) {
	xxx_messageInfo_Replacement_From.Merge(m, src)
}
func (m *Replacement_From) XXX_Size() int {
	return xxx_messageInfo_Replacement_From.Size(m)
}
func (m *Replacement_From) XXX_DiscardUnknown() {
	xxx_messageInfo_Replacement_From.DiscardUnknown(m)
}

var xxx_messageInfo_Replacement_From proto.InternalMessageInfo

type isReplacement_From_Value interface {
	isReplacement_From_Value()
}

type Replacement_From_Missing struct {
	Missing bool `protobuf:"varint,1,opt,name=missing,proto3,oneof"`
}

type Replacement_From_Datum struct {
	Datum *TypedDatum `protobuf:"bytes,2,opt,name=datum,proto3,oneof"`
}

func (*Replacement_From_Missing) isReplacement_From_Value() {}

func (*Replacement_From_Datum) isReplacement_From_Value() {}

func (m *Replacement_From) GetValue() isReplacement_From_Value {
	if m != nil {
		return m.Value
	}
	return nil
}

func (m *Replacement_From) GetMissing() bool {
	if x, ok := m.GetValue().(*Replacement_From_Missing); ok {
		return x.Missing
	}
	return false
}

func (m *Replacement_From) GetDatum() *TypedDatum {
	if x, ok := m.GetValue().(*Replacement_From_Datum); ok {
		return x.Datum
	}
	return nil
}

// XXX_OneofWrappers is for the internal use of the proto package.
func (*Replacement_From) XXX_OneofWrappers() []interface{} {
	return []interface{}{
		(*Replacement_From_Missing)(nil),
		(*Replacement_From_Datum)(nil),
	}
}

type Replacement_To struct {
	// Types that are valid to be assigned to Value:
	//	*Replacement_To_Function
	//	*Replacement_To_Datum
	Value                isReplacement_To_Value `protobuf_oneof:"value"`
	XXX_NoUnkeyedLiteral struct{}               `json:"-"`
	XXX_unrecognized     []byte                 `json:"-"`
	XXX_sizecache        int32                  `json:"-"`
}

func (m *Replacement_To) Reset()         { *m = Replacement_To{} }
func (m *Replacement_To) String() string { return proto.CompactTextString(m) }
func (*Replacement_To) ProtoMessage()    {}
func (*Replacement_To) Descriptor() ([]byte, []int) {
	return fileDescriptor_d938547f84707355, []int{8, 1}
}

func (m *Replacement_To) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_Replacement_To.Unmarshal(m, b)
}
func (m *Replacement_To) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_Replacement_To.Marshal(b, m, deterministic)
}
func (m *Replacement_To) XXX_Merge(src proto.Message) {
	xxx_messageInfo_Replacement_To.Merge(m, src)
}
func (m *Replacement_To) XXX_Size() int {
	return xxx_messageInfo_Replacement_To.Size(m)
}
func (m *Replacement_To) XXX_DiscardUnknown() {
	xxx_messageInfo_Replacement_To.DiscardUnknown(m)
}

var xxx_messageInfo_Replacement_To proto.InternalMessageInfo

type isReplacement_To_Value interface {
	isReplacement_To_Value()
}

type Replacement_To_Function struct {
	Function FunctionType `protobuf:"varint,3,opt,name=function,proto3,enum=pb.FunctionType,oneof"`
}

type Replacement_To_Datum struct {
	Datum *TypedDatum `protobuf:"bytes,4,opt,name=datum,proto3,oneof"`
}

func (*Replacement_To_Function) isReplacement_To_Value() {}

func (*Replacement_To_Datum) isReplacement_To_Value() {}

func (m *Replacement_To) GetValue() isReplacement_To_Value {
	if m != nil {
		return m.Value
	}
	return nil
}

func (m *Replacement_To) GetFunction() FunctionType {
	if x, ok := m.GetValue().(*Replacement_To_Function); ok {
		return x.Function
	}
	return FunctionType_MEAN
}

func (m *Replacement_To) GetDatum() *TypedDatum {
	if x, ok := m.GetValue().(*Replacement_To_Datum); ok {
		return x.Datum
	}
	return nil
}

// XXX_OneofWrappers is for the internal use of the proto package.
func (*Replacement_To) XXX_OneofWrappers() []interface{} {
	return []interface{}{
		(*Replacement_To_Function)(nil),
		(*Replacement_To_Datum)(nil),
	}
}

type TypedDatum struct {
	Type DataType `protobuf:"varint,1,opt,name=type,proto3,enum=pb.DataType" json:"type,omitempty"`
	// NOTE: caps here to match DataType names
	//
	// Types that are valid to be assigned to Value:
	//	*TypedDatum_BOOLEAN
	//	*TypedDatum_FLOAT
	//	*TypedDatum_INTEGER
	//	*TypedDatum_STRING
	Value                isTypedDatum_Value `protobuf_oneof:"value"`
	XXX_NoUnkeyedLiteral struct{}           `json:"-"`
	XXX_unrecognized     []byte             `json:"-"`
	XXX_sizecache        int32              `json:"-"`
}

func (m *TypedDatum) Reset()         { *m = TypedDatum{} }
func (m *TypedDatum) String() string { return proto.CompactTextString(m) }
func (*TypedDatum) ProtoMessage()    {}
func (*TypedDatum) Descriptor() ([]byte, []int) {
	return fileDescriptor_d938547f84707355, []int{9}
}

func (m *TypedDatum) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_TypedDatum.Unmarshal(m, b)
}
func (m *TypedDatum) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_TypedDatum.Marshal(b, m, deterministic)
}
func (m *TypedDatum) XXX_Merge(src proto.Message) {
	xxx_messageInfo_TypedDatum.Merge(m, src)
}
func (m *TypedDatum) XXX_Size() int {
	return xxx_messageInfo_TypedDatum.Size(m)
}
func (m *TypedDatum) XXX_DiscardUnknown() {
	xxx_messageInfo_TypedDatum.DiscardUnknown(m)
}

var xxx_messageInfo_TypedDatum proto.InternalMessageInfo

func (m *TypedDatum) GetType() DataType {
	if m != nil {
		return m.Type
	}
	return DataType_BOOLEAN
}

type isTypedDatum_Value interface {
	isTypedDatum_Value()
}

type TypedDatum_BOOLEAN struct {
	BOOLEAN bool `protobuf:"varint,2,opt,name=BOOLEAN,proto3,oneof"`
}

type TypedDatum_FLOAT struct {
	FLOAT float64 `protobuf:"fixed64,3,opt,name=FLOAT,proto3,oneof"`
}

type TypedDatum_INTEGER struct {
	INTEGER int64 `protobuf:"varint,4,opt,name=INTEGER,proto3,oneof"`
}

type TypedDatum_STRING struct {
	STRING string `protobuf:"bytes,5,opt,name=STRING,proto3,oneof"`
}

func (*TypedDatum_BOOLEAN) isTypedDatum_Value() {}

func (*TypedDatum_FLOAT) isTypedDatum_Value() {}

func (*TypedDatum_INTEGER) isTypedDatum_Value() {}

func (*TypedDatum_STRING) isTypedDatum_Value() {}

func (m *TypedDatum) GetValue() isTypedDatum_Value {
	if m != nil {
		return m.Value
	}
	return nil
}

func (m *TypedDatum) GetBOOLEAN() bool {
	if x, ok := m.GetValue().(*TypedDatum_BOOLEAN); ok {
		return x.BOOLEAN
	}
	return false
}

func (m *TypedDatum) GetFLOAT() float64 {
	if x, ok := m.GetValue().(*TypedDatum_FLOAT); ok {
		return x.FLOAT
	}
	return 0
}

func (m *TypedDatum) GetINTEGER() int64 {
	if x, ok := m.GetValue().(*TypedDatum_INTEGER); ok {
		return x.INTEGER
	}
	return 0
}

func (m *TypedDatum) GetSTRING() string {
	if x, ok := m.GetValue().(*TypedDatum_STRING); ok {
		return x.STRING
	}
	return ""
}

// XXX_OneofWrappers is for the internal use of the proto package.
func (*TypedDatum) XXX_OneofWrappers() []interface{} {
	return []interface{}{
		(*TypedDatum_BOOLEAN)(nil),
		(*TypedDatum_FLOAT)(nil),
		(*TypedDatum_INTEGER)(nil),
		(*TypedDatum_STRING)(nil),
	}
}

func init() {
	proto.RegisterType((*Attribute)(nil), "pb.Attribute")
	proto.RegisterType((*InferRequest)(nil), "pb.InferRequest")
	proto.RegisterType((*InferRequest_Parameters)(nil), "pb.InferRequest.Parameters")
	proto.RegisterType((*InferResponse)(nil), "pb.InferResponse")
	proto.RegisterType((*LearnRequest)(nil), "pb.LearnRequest")
	proto.RegisterType((*LearnResponse)(nil), "pb.LearnResponse")
	proto.RegisterType((*Metadata)(nil), "pb.Metadata")
	proto.RegisterType((*PredictRequest)(nil), "pb.PredictRequest")
	proto.RegisterType((*PredictResponse)(nil), "pb.PredictResponse")
	proto.RegisterType((*Replacement)(nil), "pb.Replacement")
	proto.RegisterType((*Replacement_From)(nil), "pb.Replacement.From")
	proto.RegisterType((*Replacement_To)(nil), "pb.Replacement.To")
	proto.RegisterType((*TypedDatum)(nil), "pb.TypedDatum")
}

func init() { proto.RegisterFile("types.proto", fileDescriptor_d938547f84707355) }

var fileDescriptor_d938547f84707355 = []byte{
	// 707 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x8c, 0x54, 0xcd, 0x6e, 0x13, 0x31,
	0x10, 0xce, 0x6e, 0x93, 0x36, 0x3b, 0x9b, 0xa4, 0x95, 0x55, 0x20, 0x4a, 0x69, 0x89, 0xf6, 0x80,
	0x02, 0x12, 0x39, 0xa4, 0x02, 0x89, 0x03, 0x87, 0x96, 0xa6, 0x4d, 0x45, 0x9b, 0x56, 0xee, 0x5e,
	0xb8, 0x10, 0x39, 0xc9, 0xb4, 0x59, 0xc8, 0xfe, 0x60, 0x3b, 0x55, 0xf3, 0x24, 0x9c, 0x79, 0x09,
	0x9e, 0x86, 0x87, 0x41, 0x76, 0xbc, 0x3f, 0x94, 0x0a, 0xb8, 0xad, 0xbf, 0x99, 0xef, 0x9b, 0x99,
	0x6f, 0xc7, 0x06, 0x57, 0x2e, 0x13, 0x14, 0xdd, 0x84, 0xc7, 0x32, 0x26, 0x76, 0x32, 0x6e, 0xb9,
	0x18, 0x2d, 0x42, 0x03, 0x78, 0x3f, 0x2c, 0x70, 0x0e, 0xa4, 0xe4, 0xc1, 0x78, 0x21, 0x91, 0x10,
	0x28, 0x47, 0x2c, 0xc4, 0xa6, 0xd5, 0xb6, 0x3a, 0x0e, 0xd5, 0xdf, 0xe4, 0x35, 0xd4, 0xc7, 0x38,
	0x63, 0xb7, 0x41, 0xcc, 0x47, 0x4a, 0xaa, 0x69, 0xb7, 0xad, 0x4e, 0xa3, 0xb7, 0xd5, 0x4d, 0xc6,
	0xdd, 0x43, 0x13, 0xf0, 0x97, 0x09, 0xd2, 0xda, 0xb8, 0x70, 0x22, 0x2f, 0xc0, 0x99, 0x32, 0xc9,
	0x56, 0x94, 0x35, 0x4d, 0xa9, 0x29, 0xca, 0x11, 0x93, 0x4c, 0xa7, 0x57, 0xa7, 0xe6, 0x8b, 0xec,
	0x43, 0x8d, 0x63, 0x32, 0x67, 0x13, 0x0c, 0x31, 0x92, 0xa2, 0x59, 0x6e, 0xaf, 0x75, 0xdc, 0xde,
	0xa6, 0xca, 0xa6, 0x39, 0x4e, 0x7f, 0x4b, 0xf2, 0x7e, 0x5a, 0x50, 0x3b, 0x8d, 0xae, 0x91, 0x53,
	0xfc, 0xba, 0x40, 0x21, 0xc9, 0x3b, 0x80, 0x84, 0x71, 0x16, 0xa2, 0x44, 0x2e, 0xf4, 0x04, 0x6e,
	0x6f, 0x47, 0x69, 0x14, 0xb3, 0xba, 0x97, 0x59, 0xca, 0xa0, 0x44, 0x0b, 0x04, 0xb2, 0x0d, 0x65,
	0xd5, 0x90, 0x9e, 0xae, 0x36, 0x28, 0x51, 0x7d, 0x6a, 0x45, 0x00, 0x39, 0x83, 0xbc, 0x81, 0x27,
	0xd1, 0x22, 0x44, 0x1e, 0x4c, 0x46, 0xc8, 0xb9, 0xf2, 0x63, 0xc6, 0x51, 0xcc, 0xe2, 0xf9, 0x54,
	0xd7, 0xb3, 0xe9, 0x23, 0x13, 0xee, 0xab, 0xa8, 0x9f, 0x06, 0x49, 0x07, 0xb6, 0xa2, 0x51, 0xc8,
	0xee, 0x46, 0x13, 0x26, 0xf1, 0x26, 0xe6, 0x01, 0x0a, 0x5d, 0xa7, 0x42, 0x1b, 0xd1, 0x39, 0xbb,
	0x7b, 0x9f, 0xa1, 0x87, 0x1b, 0x50, 0xb9, 0x65, 0xf3, 0x05, 0x7a, 0x6f, 0xa1, 0x6e, 0xfa, 0x16,
	0x49, 0x1c, 0x09, 0x24, 0x1d, 0xa8, 0x86, 0x28, 0x99, 0xee, 0x71, 0x35, 0x9c, 0xb6, 0xf3, 0xdc,
	0x60, 0x34, 0x8b, 0x7a, 0x1f, 0xa1, 0x76, 0x86, 0x8c, 0x47, 0xa9, 0x31, 0x2f, 0xff, 0xce, 0x1c,
	0x94, 0x72, 0xee, 0xc3, 0x2e, 0xe4, 0x5d, 0x7d, 0x86, 0xba, 0x91, 0x36, 0x5d, 0xed, 0x80, 0x33,
	0x57, 0xc0, 0xe8, 0x0b, 0x2e, 0xcd, 0xd6, 0x54, 0x35, 0xf0, 0x01, 0x97, 0x64, 0x0f, 0x00, 0x15,
	0x8f, 0xc9, 0x20, 0x8e, 0xb4, 0xa4, 0x43, 0x0b, 0x08, 0x79, 0x0a, 0xce, 0x14, 0x27, 0x81, 0x08,
	0xe2, 0x48, 0xe8, 0x15, 0x71, 0x68, 0x0e, 0x78, 0x4b, 0xa8, 0xa6, 0x2d, 0x92, 0x5d, 0x80, 0x19,
	0x13, 0x23, 0xc9, 0xf8, 0x0d, 0x4a, 0x5d, 0xa7, 0x4a, 0x9d, 0x19, 0x13, 0xbe, 0x06, 0xc8, 0x33,
	0x70, 0x57, 0xa1, 0x91, 0xde, 0x5e, 0x53, 0x69, 0x05, 0x0d, 0xd5, 0x0e, 0xbf, 0x02, 0x60, 0xe9,
	0x92, 0xab, 0x52, 0x6a, 0xbf, 0xea, 0xca, 0x84, 0x6c, 0xf5, 0x69, 0x21, 0xc1, 0x1b, 0x42, 0xe3,
	0x92, 0xe3, 0x34, 0x98, 0xc8, 0xd4, 0xc3, 0xdd, 0x3f, 0xe6, 0x54, 0xb6, 0x65, 0x93, 0xfe, 0xc3,
	0xb6, 0x4f, 0xb0, 0x99, 0xe9, 0x19, 0xe3, 0x3c, 0x70, 0x93, 0x15, 0xa4, 0xa7, 0x4f, 0x25, 0x8b,
	0x20, 0xd9, 0x2b, 0xfa, 0x63, 0x9b, 0x8c, 0x1c, 0xca, 0xf5, 0xbf, 0xd9, 0xe0, 0x16, 0x6e, 0x0a,
	0xe9, 0x40, 0xf9, 0x9a, 0xc7, 0xa1, 0xf9, 0xdb, 0xdb, 0xf7, 0x2e, 0x52, 0xf7, 0x98, 0xc7, 0x21,
	0xd5, 0x19, 0xc4, 0x03, 0x5b, 0xc6, 0x5a, 0xdb, 0xed, 0x91, 0xfb, 0x79, 0x7e, 0x4c, 0x6d, 0x19,
	0xb7, 0xae, 0xa0, 0xac, 0x18, 0xa4, 0x05, 0x1b, 0x61, 0x20, 0x44, 0x10, 0xdd, 0xac, 0xfe, 0xc0,
	0xa0, 0x44, 0x53, 0x80, 0x3c, 0x87, 0xca, 0x94, 0xc9, 0x45, 0x68, 0xa4, 0x1a, 0x4a, 0x4a, 0xdd,
	0xed, 0xe9, 0x91, 0x42, 0x07, 0x25, 0xba, 0x0a, 0x67, 0x2d, 0xb7, 0x10, 0x6c, 0x3f, 0x26, 0x5d,
	0xa8, 0x5e, 0x2f, 0x22, 0x3d, 0xae, 0x79, 0x23, 0xf4, 0xb3, 0x72, 0x6c, 0x30, 0xa5, 0xa0, 0x7c,
	0x4e, 0x73, 0xf2, 0x32, 0xe5, 0xff, 0x2b, 0xe3, 0x7d, 0xb7, 0x00, 0xf2, 0x04, 0xd2, 0x86, 0xb2,
	0x7e, 0x8f, 0xac, 0x07, 0xde, 0x23, 0x1d, 0x51, 0x43, 0x1e, 0x5e, 0x5c, 0x9c, 0xf5, 0x0f, 0x86,
	0x7a, 0x14, 0x3d, 0xa4, 0x01, 0xc8, 0x63, 0xa8, 0x1c, 0x9f, 0x5d, 0x1c, 0xf8, 0xba, 0x55, 0x4b,
	0x55, 0xd3, 0x47, 0xc5, 0x39, 0x1d, 0xfa, 0xfd, 0x93, 0x3e, 0xd5, 0x7d, 0xad, 0x29, 0x8e, 0x01,
	0x48, 0x13, 0xd6, 0xaf, 0x7c, 0x7a, 0x3a, 0x3c, 0x69, 0x56, 0xcc, 0x0f, 0x34, 0xe7, 0xac, 0xc7,
	0xf1, 0xba, 0x7e, 0x89, 0xf7, 0x7f, 0x05, 0x00, 0x00, 0xff, 0xff, 0x18, 0xe5, 0x2e, 0x10, 0xa9,
	0x05, 0x00, 0x00,
}