import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

/// Generates a minimal valid GLB (a single flat triangle) and copies it to
/// every `<gender>_<fatBucket>_<muscleBucket>.glb` filename the app expects,
/// so the 3D viewer pipeline can be built and tested before real MakeHuman/
/// Blender exports exist. See
/// docs/superpowers/specs/2026-07-05-vitruvian-3d-body-design.md.
void main() {
  final glb = _buildPlaceholderGlb();

  final outDir = Directory('assets/vitruvian3d');
  if (!outDir.existsSync()) {
    outDir.createSync(recursive: true);
  }

  const genders = ['male', 'female'];
  const levels = ['low', 'medium', 'high'];

  var count = 0;
  for (final gender in genders) {
    for (final fat in levels) {
      for (final muscle in levels) {
        final file = File('${outDir.path}/${gender}_${fat}_$muscle.glb');
        file.writeAsBytesSync(glb);
        count++;
      }
    }
  }

  stdout.writeln('Wrote $count placeholder GLB files to ${outDir.path}');
}

Uint8List _buildPlaceholderGlb() {
  const json = '{"asset":{"version":"2.0","generator":"placeholder"},'
      '"scene":0,"scenes":[{"nodes":[0]}],"nodes":[{"mesh":0}],'
      '"meshes":[{"primitives":[{"attributes":{"POSITION":0},"indices":1}]}],'
      '"buffers":[{"byteLength":42}],'
      '"bufferViews":['
      '{"buffer":0,"byteOffset":0,"byteLength":36,"target":34962},'
      '{"buffer":0,"byteOffset":36,"byteLength":6,"target":34963}'
      '],'
      '"accessors":['
      '{"bufferView":0,"byteOffset":0,"componentType":5126,"count":3,'
      '"type":"VEC3","max":[1.0,1.0,0.0],"min":[0.0,0.0,0.0]},'
      '{"bufferView":1,"byteOffset":0,"componentType":5123,"count":3,'
      '"type":"SCALAR"}'
      ']}';

  final jsonBytes = _padToFour(utf8.encode(json), 0x20);
  final binBytes = _padToFour(_buildTriangleBuffer(), 0x00);

  final totalLength = 12 + 8 + jsonBytes.length + 8 + binBytes.length;

  final builder = BytesBuilder();
  builder.add(_uint32(0x46546C67)); // magic: 'glTF'
  builder.add(_uint32(2)); // version
  builder.add(_uint32(totalLength));

  builder.add(_uint32(jsonBytes.length));
  builder.add(_uint32(0x4E4F534A)); // chunk type: 'JSON'
  builder.add(jsonBytes);

  builder.add(_uint32(binBytes.length));
  builder.add(_uint32(0x004E4942)); // chunk type: 'BIN\0'
  builder.add(binBytes);

  return builder.toBytes();
}

Uint8List _buildTriangleBuffer() {
  final data = ByteData(42);
  const positions = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
  for (var i = 0; i < positions.length; i++) {
    data.setFloat32(i * 4, positions[i], Endian.little);
  }
  const indices = [0, 1, 2];
  for (var i = 0; i < indices.length; i++) {
    data.setUint16(36 + i * 2, indices[i], Endian.little);
  }
  return data.buffer.asUint8List();
}

Uint8List _uint32(int value) {
  final data = ByteData(4)..setUint32(0, value, Endian.little);
  return data.buffer.asUint8List();
}

Uint8List _padToFour(List<int> bytes, int padByte) {
  final remainder = bytes.length % 4;
  if (remainder == 0) return Uint8List.fromList(bytes);
  final padding = 4 - remainder;
  return Uint8List.fromList([...bytes, ...List.filled(padding, padByte)]);
}
