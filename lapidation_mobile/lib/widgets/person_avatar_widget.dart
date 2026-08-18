import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../../models/person.dart';
import '../providers/person_provider.dart';
// ─────────────────────────────────────────────────────────────────────────────
// Person avatar for create-post header
// ─────────────────────────────────────────────────────────────────────────────

const _businessAvatarPlaceholder = 'assets/images/avatar_personal_trainer.png';

class PersonAvatar extends StatelessWidget {
  final Person? person;
  const PersonAvatar({super.key, this.person});

  @override
  Widget build(BuildContext context) {
    final personProvider = context.watch<PersonProvider>();
    final businessLogo = personProvider.activeBusinessProfile?.logo;

    if (personProvider.isProfessional) {
      return CircleAvatar(
        radius: 20,
        child: ClipOval(
          child: (businessLogo != null && businessLogo.isNotEmpty)
              ? CachedNetworkImage(
                  imageUrl: businessLogo,
                  width: 40,
                  height: 40,
                  fit: BoxFit.cover,
                  placeholder: (ctx, url) =>
                      Image.asset(_businessAvatarPlaceholder, fit: BoxFit.cover),
                  errorWidget: (ctx, url, err) => Image.asset(
                    _businessAvatarPlaceholder,
                    width: 40,
                    height: 40,
                    fit: BoxFit.cover,
                  ),
                )
              : Image.asset(
                  _businessAvatarPlaceholder,
                  width: 40,
                  height: 40,
                  fit: BoxFit.cover,
                ),
        ),
      );
    }

    final isFemale = person?.gender?.toLowerCase() == 'female';
    final asset = isFemale
        ? 'assets/images/avatar_female.png'
        : 'assets/images/avatar_male.png';

    if (person?.avatar != null && person!.avatar!.isNotEmpty) {
      return CircleAvatar(
        radius: 20,
        child: ClipOval(
          child: CachedNetworkImage(
            imageUrl: person!.avatar!,
            width: 40,
            height: 40,
            fit: BoxFit.cover,
            placeholder: (ctx, url) => Image.asset(asset, fit: BoxFit.cover),
            errorWidget: (ctx, url, err) =>
                Image.asset(asset, width: 40, height: 40, fit: BoxFit.cover),
          ),
        ),
      );
    }

    return CircleAvatar(
      radius: 20,
      child: ClipOval(
        child: Image.asset(asset, width: 40, height: 40, fit: BoxFit.cover),
      ),
    );
  }
}
