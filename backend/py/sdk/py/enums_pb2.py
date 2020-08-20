# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: enums.proto

import sys
_b=sys.version_info[0]<3 and (lambda x:x) or (lambda x:x.encode('latin1'))
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='enums.proto',
  package='pb',
  syntax='proto3',
  serialized_options=None,
  serialized_pb=_b('\n\x0b\x65nums.proto\x12\x02pb*9\n\x0c\x42\x65haviorType\x12\x0e\n\nCONTINUOUS\x10\x00\x12\x0f\n\x0b\x43\x41TEGORICAL\x10\x01\x12\x08\n\x04TEXT\x10\x02*;\n\x08\x44\x61taType\x12\x0b\n\x07\x42OOLEAN\x10\x00\x12\t\n\x05\x46LOAT\x10\x01\x12\x0b\n\x07INTEGER\x10\x02\x12\n\n\x06STRING\x10\x03*.\n\x0c\x46unctionType\x12\x08\n\x04MEAN\x10\x00\x12\n\n\x06MEDIAN\x10\x01\x12\x08\n\x04MODE\x10\x02*D\n\x07KeyType\x12\t\n\x05LEARN\x10\x00\x12\x0f\n\x0bTRANSFORMER\x10\x01\x12\r\n\tESTIMATOR\x10\x02\x12\x0e\n\nEVALUATION\x10\x03*5\n\x0bServiceType\x12\x11\n\rSERVICE_LEARN\x10\x00\x12\x13\n\x0fSERVICE_PREDICT\x10\x01*2\n\x08TaskType\x12\x0c\n\x08\x43LASSIFY\x10\x00\x12\x0b\n\x07REGRESS\x10\x01\x12\x0b\n\x07\x43LUSTER\x10\x02\x62\x06proto3')
)

_BEHAVIORTYPE = _descriptor.EnumDescriptor(
  name='BehaviorType',
  full_name='pb.BehaviorType',
  filename=None,
  file=DESCRIPTOR,
  values=[
    _descriptor.EnumValueDescriptor(
      name='CONTINUOUS', index=0, number=0,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='CATEGORICAL', index=1, number=1,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='TEXT', index=2, number=2,
      serialized_options=None,
      type=None),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=19,
  serialized_end=76,
)
_sym_db.RegisterEnumDescriptor(_BEHAVIORTYPE)

BehaviorType = enum_type_wrapper.EnumTypeWrapper(_BEHAVIORTYPE)
_DATATYPE = _descriptor.EnumDescriptor(
  name='DataType',
  full_name='pb.DataType',
  filename=None,
  file=DESCRIPTOR,
  values=[
    _descriptor.EnumValueDescriptor(
      name='BOOLEAN', index=0, number=0,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='FLOAT', index=1, number=1,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='INTEGER', index=2, number=2,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='STRING', index=3, number=3,
      serialized_options=None,
      type=None),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=78,
  serialized_end=137,
)
_sym_db.RegisterEnumDescriptor(_DATATYPE)

DataType = enum_type_wrapper.EnumTypeWrapper(_DATATYPE)
_FUNCTIONTYPE = _descriptor.EnumDescriptor(
  name='FunctionType',
  full_name='pb.FunctionType',
  filename=None,
  file=DESCRIPTOR,
  values=[
    _descriptor.EnumValueDescriptor(
      name='MEAN', index=0, number=0,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='MEDIAN', index=1, number=1,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='MODE', index=2, number=2,
      serialized_options=None,
      type=None),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=139,
  serialized_end=185,
)
_sym_db.RegisterEnumDescriptor(_FUNCTIONTYPE)

FunctionType = enum_type_wrapper.EnumTypeWrapper(_FUNCTIONTYPE)
_KEYTYPE = _descriptor.EnumDescriptor(
  name='KeyType',
  full_name='pb.KeyType',
  filename=None,
  file=DESCRIPTOR,
  values=[
    _descriptor.EnumValueDescriptor(
      name='LEARN', index=0, number=0,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='TRANSFORMER', index=1, number=1,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='ESTIMATOR', index=2, number=2,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='EVALUATION', index=3, number=3,
      serialized_options=None,
      type=None),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=187,
  serialized_end=255,
)
_sym_db.RegisterEnumDescriptor(_KEYTYPE)

KeyType = enum_type_wrapper.EnumTypeWrapper(_KEYTYPE)
_SERVICETYPE = _descriptor.EnumDescriptor(
  name='ServiceType',
  full_name='pb.ServiceType',
  filename=None,
  file=DESCRIPTOR,
  values=[
    _descriptor.EnumValueDescriptor(
      name='SERVICE_LEARN', index=0, number=0,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='SERVICE_PREDICT', index=1, number=1,
      serialized_options=None,
      type=None),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=257,
  serialized_end=310,
)
_sym_db.RegisterEnumDescriptor(_SERVICETYPE)

ServiceType = enum_type_wrapper.EnumTypeWrapper(_SERVICETYPE)
_TASKTYPE = _descriptor.EnumDescriptor(
  name='TaskType',
  full_name='pb.TaskType',
  filename=None,
  file=DESCRIPTOR,
  values=[
    _descriptor.EnumValueDescriptor(
      name='CLASSIFY', index=0, number=0,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='REGRESS', index=1, number=1,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='CLUSTER', index=2, number=2,
      serialized_options=None,
      type=None),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=312,
  serialized_end=362,
)
_sym_db.RegisterEnumDescriptor(_TASKTYPE)

TaskType = enum_type_wrapper.EnumTypeWrapper(_TASKTYPE)
CONTINUOUS = 0
CATEGORICAL = 1
TEXT = 2
BOOLEAN = 0
FLOAT = 1
INTEGER = 2
STRING = 3
MEAN = 0
MEDIAN = 1
MODE = 2
LEARN = 0
TRANSFORMER = 1
ESTIMATOR = 2
EVALUATION = 3
SERVICE_LEARN = 0
SERVICE_PREDICT = 1
CLASSIFY = 0
REGRESS = 1
CLUSTER = 2


DESCRIPTOR.enum_types_by_name['BehaviorType'] = _BEHAVIORTYPE
DESCRIPTOR.enum_types_by_name['DataType'] = _DATATYPE
DESCRIPTOR.enum_types_by_name['FunctionType'] = _FUNCTIONTYPE
DESCRIPTOR.enum_types_by_name['KeyType'] = _KEYTYPE
DESCRIPTOR.enum_types_by_name['ServiceType'] = _SERVICETYPE
DESCRIPTOR.enum_types_by_name['TaskType'] = _TASKTYPE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)


# @@protoc_insertion_point(module_scope)
