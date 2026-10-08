// This is a generated file - do not edit.
//
// Generated from legal.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports
// ignore_for_file: unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use listLegalDocumentsRequestDescriptor instead')
const ListLegalDocumentsRequest$json = {
  '1': 'ListLegalDocumentsRequest',
};

/// Descriptor for `ListLegalDocumentsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listLegalDocumentsRequestDescriptor =
    $convert.base64Decode('ChlMaXN0TGVnYWxEb2N1bWVudHNSZXF1ZXN0');

@$core.Deprecated('Use listLegalDocumentsResponseDescriptor instead')
const ListLegalDocumentsResponse$json = {
  '1': 'ListLegalDocumentsResponse',
  '2': [
    {
      '1': 'documents',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.legal.LegalDocument',
      '10': 'documents'
    },
  ],
};

/// Descriptor for `ListLegalDocumentsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listLegalDocumentsResponseDescriptor =
    $convert.base64Decode(
        'ChpMaXN0TGVnYWxEb2N1bWVudHNSZXNwb25zZRI3Cglkb2N1bWVudHMYASADKAsyGS5ncnBjLm'
        'xlZ2FsLkxlZ2FsRG9jdW1lbnRSCWRvY3VtZW50cw==');

@$core.Deprecated('Use getLegalDocumentRequestDescriptor instead')
const GetLegalDocumentRequest$json = {
  '1': 'GetLegalDocumentRequest',
  '2': [
    {'1': 'document', '3': 1, '4': 1, '5': 9, '10': 'document'},
  ],
};

/// Descriptor for `GetLegalDocumentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getLegalDocumentRequestDescriptor =
    $convert.base64Decode(
        'ChdHZXRMZWdhbERvY3VtZW50UmVxdWVzdBIaCghkb2N1bWVudBgBIAEoCVIIZG9jdW1lbnQ=');

@$core.Deprecated('Use legalDocumentDescriptor instead')
const LegalDocument$json = {
  '1': 'LegalDocument',
  '2': [
    {'1': 'document', '3': 1, '4': 1, '5': 9, '10': 'document'},
    {'1': 'version', '3': 2, '4': 1, '5': 9, '10': 'version'},
    {'1': 'title', '3': 3, '4': 1, '5': 9, '10': 'title'},
    {'1': 'content', '3': 4, '4': 1, '5': 9, '10': 'content'},
  ],
};

/// Descriptor for `LegalDocument`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List legalDocumentDescriptor = $convert.base64Decode(
    'Cg1MZWdhbERvY3VtZW50EhoKCGRvY3VtZW50GAEgASgJUghkb2N1bWVudBIYCgd2ZXJzaW9uGA'
    'IgASgJUgd2ZXJzaW9uEhQKBXRpdGxlGAMgASgJUgV0aXRsZRIYCgdjb250ZW50GAQgASgJUgdj'
    'b250ZW50');
