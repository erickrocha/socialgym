# socialgym_mobile

A new Flutter project.

## Getting Started

This project is a starting point for a Flutter application.

A few resources to get you started if this is your first Flutter project:

- [Learn Flutter](https://docs.flutter.dev/get-started/learn-flutter)
- [Write your first Flutter app](https://docs.flutter.dev/get-started/codelab)
- [Flutter learning resources](https://docs.flutter.dev/reference/learning-resources)

For help getting started with Flutter development, view the
[online documentation](https://docs.flutter.dev/), which offers tutorials,
samples, guidance on mobile development, and a full API reference.

## gRPC proto workflow

- Keep source `.proto` files in `proto/`.
- Run the helper script to generate Dart gRPC stubs into `lib/src/generated/grpc/`.

```bash
./tool/generate_proto.sh
```

## Firebase push setup

Configure Firebase Cloud Messaging for Android and iOS outside source control. Supply these
compile-time values to the mobile build: `FIREBASE_API_KEY`, `FIREBASE_PROJECT_ID`,
`FIREBASE_MESSAGING_SENDER_ID`, `FIREBASE_ANDROID_APP_ID`, `FIREBASE_IOS_APP_ID`, and
`FIREBASE_IOS_BUNDLE_ID`. The Firebase project must have APNs configured for iOS. Push stays
disabled when required values are missing; no provider credentials or project IDs belong in Git.

