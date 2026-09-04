// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for English (`en`).
class AppLocalizationsEn extends AppLocalizations {
  AppLocalizationsEn([String locale = 'en']) : super(locale);

  @override
  String get appTitle => 'Social Gym';

  @override
  String get signInTitle => 'Log In';

  @override
  String get signInEmail => 'Email';

  @override
  String get signInPassword => 'Password';

  @override
  String get signInSubmit => 'Sign In';

  @override
  String get signInForgotPassword => 'Forgot password?';

  @override
  String get signInCreateAccount => 'Create new account';

  @override
  String get signInOr => 'or';

  @override
  String get signUpTitle => 'Create a new account';

  @override
  String get signUpFirstName => 'First Name';

  @override
  String get signUpSurname => 'Surname';

  @override
  String get signUpBirthdayMonth => 'Month';

  @override
  String get signUpBirthdayDay => 'Day';

  @override
  String get signUpBirthdayYear => 'Year';

  @override
  String get signUpGender => 'Gender';

  @override
  String get signUpGenderFemale => 'Female';

  @override
  String get signUpGenderMale => 'Male';

  @override
  String get signUpGenderCustom => 'Custom';

  @override
  String get signUpGenderCustomPlaceholder => 'Please specify';

  @override
  String get signUpMobileOrEmail => 'Mobile number or email';

  @override
  String get signUpPassword => 'New password';

  @override
  String get signUpPasswordHint =>
      'Use a combination of at least six numbers, letters and punctuation marks.';

  @override
  String get signUpSubmit => 'Sign Up';

  @override
  String get signUpAlreadyHaveAccount => 'Already have an account?';

  @override
  String get validationEmailRequired => 'Please enter your email';

  @override
  String get validationPasswordRequired => 'Please enter your password';

  @override
  String get validationPasswordMinLength =>
      'Password must be at least 6 characters';

  @override
  String get validationFirstNameRequired => 'Please enter your first name';

  @override
  String get validationSurnameRequired => 'Please enter your surname';

  @override
  String get validationDateOfBirthRequired =>
      'Please select your date of birth';

  @override
  String get validationGenderRequired => 'Please select your gender';

  @override
  String get connectionError => 'Connection error. Please try again.';

  @override
  String get homeWelcome => 'Welcome to Social Gym!';

  @override
  String get homeTitle => 'Social Gym';

  @override
  String get logout => 'Logout';

  @override
  String get monthJanuary => 'January';

  @override
  String get monthFebruary => 'February';

  @override
  String get monthMarch => 'March';

  @override
  String get monthApril => 'April';

  @override
  String get monthMay => 'May';

  @override
  String get monthJune => 'June';

  @override
  String get monthJuly => 'July';

  @override
  String get monthAugust => 'August';

  @override
  String get monthSeptember => 'September';

  @override
  String get monthOctober => 'October';

  @override
  String get monthNovember => 'November';

  @override
  String get monthDecember => 'December';

  @override
  String get menuOpen => 'Open menu';

  @override
  String get menuHome => 'Home';

  @override
  String get menuFeed => 'Feed';

  @override
  String get menuGallery => 'Gallery';

  @override
  String get menuProfile => 'Profile';

  @override
  String get menuTeam => 'Team';

  @override
  String get menuFollowers => 'Followers';

  @override
  String get menuFriends => 'Friends';

  @override
  String get menuMessages => 'Messages';

  @override
  String get menuNotifications => 'Notifications';

  @override
  String get menuWorkouts => 'Workouts';

  @override
  String get menuWorkoutSessions => 'Workout Sessions';

  @override
  String get menuEvolution => 'Evolution';

  @override
  String get menuSettings => 'Settings';

  @override
  String get menuLanguage => 'Language';

  @override
  String get menuCollapse => 'Collapse';

  @override
  String get menuOnline => 'Online';

  @override
  String get menuOffline => 'Offline';

  @override
  String get searchPlaceholder => 'Search Social Gym';

  @override
  String get comingSoon => 'Coming soon';

  @override
  String get buttonCancel => 'Cancel';

  @override
  String get buttonConfirm => 'Confirm';

  @override
  String get buttonSave => 'Save';

  @override
  String get validationRequired => 'This field is required';

  @override
  String get permissionRequired => 'Permission Required';

  @override
  String get cameraPermissionDenied =>
      'Camera access is required to take photos. Please enable it in your device settings.';

  @override
  String get photosPermissionDenied =>
      'Photo library access is required to select images. Please enable it in your device settings.';

  @override
  String get openSettings => 'Open Settings';

  @override
  String get settingsTitle => 'Settings';

  @override
  String get settingsLanguage => 'Language';

  @override
  String get settingsNotificationsEnabled => 'Enable notifications';

  @override
  String get settingsContextMenuPosition => 'Sidebar menu position';

  @override
  String get settingsContextMenuPositionLeft => 'Left';

  @override
  String get settingsContextMenuPositionTop => 'Top';

  @override
  String get settingsContextMenuPositionRight => 'Right';

  @override
  String get settingsContextMenuPositionBottom => 'Bottom';

  @override
  String get settingsHomePage => 'Home page';

  @override
  String get settingsHomePageFeed => 'Feed';

  @override
  String get settingsHomePageGallery => 'Gallery';

  @override
  String get settingsSaveSuccess => 'Settings saved';

  @override
  String get settingsSaveError => 'Could not save settings';

  @override
  String get settingsLoadError => 'Could not refresh settings from the server';

  @override
  String get settingsDangerZoneTitle => 'Danger Zone';

  @override
  String get settingsDeleteAccountDescription =>
      'Deleting your account removes your profile, workouts, posts, and all associated data.';

  @override
  String get settingsDeleteAccountButton => 'Delete Account';

  @override
  String get settingsDeleteAccountConfirmTitle => 'Delete Account';

  @override
  String get settingsDeleteAccountConfirmBody =>
      'This can\'t be undone once completed. Choose when you\'d like this to happen:';

  @override
  String get settingsDeleteAccountOptionGracePeriod =>
      'Delete after 30 days (you can cancel by logging back in)';

  @override
  String get settingsDeleteAccountOptionImmediate => 'Delete immediately';

  @override
  String get settingsDeleteAccountConfirmButton => 'Delete Account';

  @override
  String get settingsDeleteAccountScheduledTitle =>
      'Account Deletion Scheduled';

  @override
  String settingsDeleteAccountScheduledBody(String date) {
    return 'Your account will be permanently deleted on $date. You\'ve been signed out.';
  }

  @override
  String get settingsDeleteAccountError =>
      'Failed to delete account. Please try again.';

  @override
  String get signInPendingDeletionTitle => 'Account Scheduled for Deletion';

  @override
  String signInPendingDeletionBody(String date) {
    return 'Your account is scheduled to be permanently deleted on $date. Would you like to keep your account?';
  }

  @override
  String get signInPendingDeletionKeepButton => 'Keep My Account';

  @override
  String get signInPendingDeletionDismissButton => 'Not Now';

  @override
  String get signInPendingDeletionCancelSuccess =>
      'Account deletion cancelled. Welcome back!';

  @override
  String get signInPendingDeletionCancelError =>
      'Failed to cancel account deletion. Please try again.';

  @override
  String get workoutTitle => 'My Workouts';

  @override
  String get workoutDescription => 'Manage your workout routines and exercises';

  @override
  String get workoutAddNew => 'Add Workout';

  @override
  String get workoutEditTitle => 'Edit Workout';

  @override
  String get workoutStartSessionTitle => 'Start Workout Session';

  @override
  String get workoutDeleteConfirmTitle => 'Delete Workout';

  @override
  String get workoutDeleteConfirm =>
      'Are you sure you want to delete this workout?';

  @override
  String get workoutExercises => 'exercises';

  @override
  String get workoutExercisesFor => 'Exercises for';

  @override
  String get workoutNoWorkouts => 'No workouts yet. Create your first one!';

  @override
  String get workoutNoExercises => 'No exercises yet';

  @override
  String get workoutAddExercise => 'Add Exercise';

  @override
  String get workoutAddExerciseHint => 'Add exercises to start training';

  @override
  String get workoutFormName => 'Name';

  @override
  String get workoutFormNamePlaceholder => 'Workout name';

  @override
  String get workoutFormDescription => 'Description';

  @override
  String get workoutFormDescriptionPlaceholder => 'Brief description';

  @override
  String get workoutDifficulty => 'Difficulty';

  @override
  String get workoutVisibility => 'Visibility';

  @override
  String get workoutMuscleGroups => 'Muscle Groups';

  @override
  String get workoutAssignToTeamMember => 'Create for';

  @override
  String get workoutAssignToMyself => 'Myself';

  @override
  String get workoutSelectTeamMember => 'Select team member';

  @override
  String get workoutNoTeamMembers => 'You have no accepted team members yet';

  @override
  String get workoutSets => 'Sets';

  @override
  String get workoutReps => 'Reps';

  @override
  String get workoutWeight => 'Weight';

  @override
  String get workoutWeightUnit => 'kg';

  @override
  String get workoutExerciseName => 'Exercise Name';

  @override
  String get workoutExerciseNamePlaceholder => 'e.g. Bench Press';

  @override
  String get workoutExerciseDescription => 'Description';

  @override
  String get workoutExerciseDescriptionPlaceholder => 'Exercise description';

  @override
  String get workoutExercisesList => 'Exercises to add';

  @override
  String get difficultySoft => 'Soft';

  @override
  String get difficultyEasy => 'Easy';

  @override
  String get difficultyMedium => 'Medium';

  @override
  String get difficultyHard => 'Hard';

  @override
  String get difficultyStrong => 'Strong';

  @override
  String get visibilityPrivate => 'Private';

  @override
  String get visibilityFriends => 'Friends';

  @override
  String get visibilityPublic => 'Public';

  @override
  String get visibilityProfessional => 'Professional';

  @override
  String get muscleGroupChest => 'Chest';

  @override
  String get muscleGroupLegs => 'Legs';

  @override
  String get muscleGroupBack => 'Back';

  @override
  String get muscleGroupCore => 'Core';

  @override
  String get muscleGroupFullBody => 'Full Body';

  @override
  String get muscleGroupShoulders => 'Shoulders';

  @override
  String get muscleGroupArms => 'Arms';

  @override
  String get muscleGroupGlutes => 'Glutes';

  @override
  String get executionStartWorkout => 'Start Workout';

  @override
  String get executionExercise => 'Exercise';

  @override
  String get executionSet => 'Set';

  @override
  String get executionOf => 'of';

  @override
  String get executionConfirmSet => 'Confirm Set';

  @override
  String get executionSkip => 'Skip';

  @override
  String get executionKeepGoing => 'Keep going! You\'re doing great!';

  @override
  String get executionAlmostDone => 'Almost done! Last effort!';

  @override
  String get executionCompletedSets => 'Completed Sets';

  @override
  String get executionViewAll => 'View all';

  @override
  String get executionWorkoutComplete => 'Workout Complete!';

  @override
  String get executionDuration => 'Duration';

  @override
  String get executionSetsCompleted => 'Sets Completed';

  @override
  String get executionTotalVolume => 'Total Volume';

  @override
  String get executionGreatJob => 'Great job! You crushed this workout! 💪';

  @override
  String get executionClose => 'Close';

  @override
  String get executionSaving => 'Saving...';

  @override
  String get executionSaveSession => 'Save Session';

  @override
  String get executionQuitTitle => 'Quit Workout?';

  @override
  String get executionQuitMessage =>
      'Your progress will be lost. Are you sure?';

  @override
  String get executionQuitConfirm => 'Quit';

  @override
  String get profileTitle => 'Profile';

  @override
  String get profileEditProfile => 'Edit Profile';

  @override
  String get profileChangeCover => 'Change Cover';

  @override
  String get profileChangeAvatar => 'Change Avatar';

  @override
  String get profilePersonalInfo => 'Personal Information';

  @override
  String get profileFirstName => 'First Name';

  @override
  String get profileSurname => 'Surname';

  @override
  String get profileDateOfBirth => 'Date of Birth';

  @override
  String get profileGender => 'Gender';

  @override
  String get profileBiography => 'Biography';

  @override
  String get profileBiographyHint => 'Tell us about yourself...';

  @override
  String get profileJob => 'Job';

  @override
  String get profileJobHint => 'What do you do?';

  @override
  String get profileRelationship => 'Relationship Status';

  @override
  String get profileHomeTown => 'Hometown';

  @override
  String get profileCurrentCity => 'Current City';

  @override
  String get profilePhysicalStats => 'Physical Stats';

  @override
  String get profileWeight => 'Weight (kg)';

  @override
  String get profileHeight => 'Height (cm)';

  @override
  String get profileSaveChanges => 'Save Changes';

  @override
  String get profileUpdateSuccess => 'Profile updated successfully';

  @override
  String get profileUpdateError => 'Failed to update profile';

  @override
  String get profileUploadingImage => 'Uploading image...';

  @override
  String get profileImageUploadSuccess => 'Image uploaded successfully';

  @override
  String get profileImageUploadError => 'Failed to upload image';

  @override
  String get profileSelectImage => 'Select Image';

  @override
  String get profileTakePhoto => 'Take Photo';

  @override
  String get profileChooseFromGallery => 'Choose from Gallery';

  @override
  String get profileRemovePhoto => 'Remove Photo';

  @override
  String get profileGenderMale => 'Male';

  @override
  String get profileGenderFemale => 'Female';

  @override
  String get profileGenderOther => 'Other';

  @override
  String get profileRelationshipSingle => 'Single';

  @override
  String get profileRelationshipInRelationship => 'In a Relationship';

  @override
  String get profileRelationshipEngaged => 'Engaged';

  @override
  String get profileRelationshipMarried => 'Married';

  @override
  String get profileRelationshipComplicated => 'It\'s Complicated';

  @override
  String get profileMemberSince => 'Member since';

  @override
  String get addressSection => 'Addresses';

  @override
  String get addressAdd => 'Add Address';

  @override
  String get addressEdit => 'Edit Address';

  @override
  String get addressDelete => 'Delete Address';

  @override
  String get addressCurrent => 'Current Address';

  @override
  String get addressCountry => 'Country';

  @override
  String get addressPostalCode => 'Postal Code';

  @override
  String get addressLocality => 'City';

  @override
  String get addressLine1 => 'Address Line 1';

  @override
  String get addressLine2 => 'Address Line 2';

  @override
  String get addressAdministrativeArea => 'State/Province';

  @override
  String get addressSelectCountry => 'Select country';

  @override
  String get addressLocationServicesDisabled =>
      'Location services are disabled. Please enable them.';

  @override
  String get addressLocationPermissionDenied => 'Location permission denied.';

  @override
  String get addressLocationPermissionDeniedForever =>
      'Location permissions are permanently denied. Please enable them in your device settings.';

  @override
  String get addressLocationError => 'Failed to get current location.';

  @override
  String get addressNoAddresses => 'No addresses added';

  @override
  String get addressNoAddressesHint => 'Add your first address';

  @override
  String get addressAddSuccess => 'Address added successfully';

  @override
  String get addressUpdateSuccess => 'Address updated successfully';

  @override
  String get addressDeleteSuccess => 'Address deleted successfully';

  @override
  String get addressSetCurrentSuccess => 'Address set as current';

  @override
  String get addressActionError => 'Action failed. Please try again.';

  @override
  String get addressDeleteConfirm =>
      'Are you sure you want to delete this address?';

  @override
  String get addressDeleteConfirmTitle => 'Delete Address';

  @override
  String get friendsTitle => 'Friends';

  @override
  String get friendsDescription => 'Connect with other gym enthusiasts';

  @override
  String get friendsTabAll => 'Friends';

  @override
  String get friendsTabRequests => 'Requests';

  @override
  String get friendsTabSuggestions => 'Suggestions';

  @override
  String get friendsTabFind => 'Find';

  @override
  String get friendsNoFriends => 'No friends yet';

  @override
  String get friendsNoFriendsHint =>
      'Start connecting with other gym enthusiasts';

  @override
  String get friendsNoRequests => 'No friend requests';

  @override
  String get friendsNoRequestsHint => 'You don\'t have any pending requests';

  @override
  String get friendsNoSuggestions => 'No suggestions available';

  @override
  String get friendsNoSuggestionsHint => 'Check back later for new suggestions';

  @override
  String get friendsAddFriend => 'Add Friend';

  @override
  String get friendsAccept => 'Accept';

  @override
  String get friendsReject => 'Reject';

  @override
  String get friendsCancel => 'Cancel Request';

  @override
  String get friendsRemove => 'Remove Friend';

  @override
  String get friendsPending => 'Pending';

  @override
  String get friendsReceivedRequests => 'Received Requests';

  @override
  String get friendsSentRequests => 'Sent Requests';

  @override
  String get friendsRequestSent => 'Friend request sent';

  @override
  String get friendsRequestAccepted => 'Friend request accepted';

  @override
  String get friendsRequestRejected => 'Friend request rejected';

  @override
  String get friendsRequestCancelled => 'Friend request cancelled';

  @override
  String get friendsRemoved => 'Friend removed';

  @override
  String get friendsActionError => 'Action failed. Please try again.';

  @override
  String get friendsLocationAnywhere => 'Anywhere';

  @override
  String get friendsLocationSavedAddress => 'Near my address';

  @override
  String get friendsLocationCurrentGps => 'Near me now';

  @override
  String get friendsFindSearchHint => 'Search by name or username';

  @override
  String get friendsFindPromptTitle => 'Find friends';

  @override
  String get friendsFindPromptHint =>
      'Search by name or use a location filter to find people';

  @override
  String get friendsFindNoResults => 'No people found';

  @override
  String get friendsFindNoResultsHint =>
      'Try a different name or location filter';

  @override
  String get friendViewProfile => 'View Profile';

  @override
  String get profileFriendLoadError =>
      'Could not load full profile. Basic info is shown.';

  @override
  String get teamTitle => 'Team';

  @override
  String get teamDescription => 'Manage your business team and memberships';

  @override
  String get teamTabMembers => 'Members';

  @override
  String get teamTabSentInvites => 'Sent Invites';

  @override
  String get teamTabMyTeams => 'My Teams';

  @override
  String get teamTabReceivedInvites => 'Received Invites';

  @override
  String get teamNoMembers => 'No team members yet';

  @override
  String get teamNoMembersHint => 'Invite people to join your team';

  @override
  String get teamNoSentInvites => 'No pending invites';

  @override
  String get teamNoSentInvitesHint => 'Invites you send will appear here';

  @override
  String get teamNoTeams => 'You haven\'t joined any team';

  @override
  String get teamNoTeamsHint => 'Businesses you join will appear here';

  @override
  String get teamNoReceivedInvites => 'No invites received';

  @override
  String get teamNoReceivedInvitesHint =>
      'Invitations from businesses will appear here';

  @override
  String get teamInvite => 'Invite';

  @override
  String get teamInviteSearchHint => 'Search by name or email';

  @override
  String get teamInviteSend => 'Send Invite';

  @override
  String get teamInviteNoResults => 'No people found';

  @override
  String get teamAccept => 'Accept';

  @override
  String get teamDeny => 'Deny';

  @override
  String get teamCancel => 'Cancel';

  @override
  String get teamRequestSent => 'Invite sent';

  @override
  String get teamRequestAccepted => 'Invite accepted';

  @override
  String get teamRequestDenied => 'Invite denied';

  @override
  String get teamRequestCancelled => 'Invite cancelled';

  @override
  String get teamActionError => 'Action failed. Please try again.';

  @override
  String get workoutInvitesTitle => 'Workout Invites';

  @override
  String get workoutInvitesDescription =>
      'Workouts assigned to you and to your team';

  @override
  String get workoutInvitesMenu => 'Workout Invites';

  @override
  String get workoutInvitesTabReceived => 'Received';

  @override
  String get workoutInvitesTabSent => 'Sent';

  @override
  String workoutInviteFrom(String name) {
    return 'From $name';
  }

  @override
  String workoutInviteTo(String name) {
    return 'To $name';
  }

  @override
  String get workoutInviteFromTrainer => 'your trainer';

  @override
  String get workoutInviteAccept => 'Accept';

  @override
  String get workoutInviteReject => 'Reject';

  @override
  String get workoutInviteCancel => 'Cancel';

  @override
  String get workoutInviteAccepted => 'Workout accepted';

  @override
  String get workoutInviteRejected => 'Workout rejected';

  @override
  String get workoutInviteCancelled => 'Assignment cancelled';

  @override
  String get workoutInviteActionError => 'Action failed. Please try again.';

  @override
  String get workoutInviteNoReceived => 'No workout invites';

  @override
  String get workoutInviteNoReceivedHint =>
      'Workouts a trainer assigns you will appear here';

  @override
  String get workoutInviteNoSent => 'No assignments sent';

  @override
  String get workoutInviteNoSentHint =>
      'Workouts you assign to team members will appear here';

  @override
  String get workoutStatusPending => 'Pending';

  @override
  String get workoutStatusAccepted => 'Accepted';

  @override
  String get workoutStatusRejected => 'Rejected';

  @override
  String get workoutStatusCancelled => 'Cancelled';

  @override
  String get exerciseOwnerName => 'Owner Name';

  @override
  String get exerciseCategory => 'Category';

  @override
  String get categoryForce => 'Force';

  @override
  String get categoryCardio => 'Cardio';

  @override
  String get workoutDuration => 'Duration';

  @override
  String get sessionsTitle => 'Workout Sessions';

  @override
  String get sessionsCount => 'Sessions';

  @override
  String get sessionsFilterLastWeek => 'Last Week';

  @override
  String get sessionsFilterLastMonth => 'Last Month';

  @override
  String get sessionsFilterLast3Months => 'Last 3 Months';

  @override
  String get sessionsFilterCustom => 'Custom Range';

  @override
  String get sessionsFilterStartDate => 'Start Date';

  @override
  String get sessionsFilterEndDate => 'End Date';

  @override
  String get sessionsNoData => 'No sessions found for the selected period.';

  @override
  String get sessionsViewProgress => 'View My Progress';

  @override
  String get sessionsChartVolumeTitle => 'Volume per Session (kg)';

  @override
  String get dayMonday => 'Monday';

  @override
  String get dayTuesday => 'Tuesday';

  @override
  String get dayWednesday => 'Wednesday';

  @override
  String get dayThursday => 'Thursday';

  @override
  String get dayFriday => 'Friday';

  @override
  String get daySaturday => 'Saturday';

  @override
  String get daySunday => 'Sunday';

  @override
  String get feedTitle => 'Feed';

  @override
  String get feedCreatePost => 'Create Post';

  @override
  String get feedWriteSomething => 'What\'s on your mind?';

  @override
  String get feedPostButton => 'Post';

  @override
  String get feedLiveVideo => 'Live Video';

  @override
  String get feedPhotoVideo => 'Photo/Video';

  @override
  String get feedFeeling => 'Feeling';

  @override
  String get feedLike => 'Like';

  @override
  String get feedLove => 'Love';

  @override
  String get feedHaha => 'Haha';

  @override
  String get feedWow => 'Wow';

  @override
  String get feedSad => 'Sad';

  @override
  String get feedAngry => 'Angry';

  @override
  String get feedComment => 'Comment';

  @override
  String get feedShare => 'Share';

  @override
  String get feedAddComment => 'Write a comment...';

  @override
  String get feedNoComments => 'No comments yet';

  @override
  String get feedNoPostsYet => 'No posts yet';

  @override
  String get feedStartSharing =>
      'Be the first to share something with your gym community!';

  @override
  String get feedPostSuccess => 'Post created successfully!';

  @override
  String get feedPostError => 'Failed to create post. Please try again.';

  @override
  String get feedLoadError => 'Failed to load feed. Please try again.';

  @override
  String get feedComments => 'comments';

  @override
  String get feedReactions => 'reactions';

  @override
  String get feedAddPhoto => 'Photo';

  @override
  String get feedAddVideo => 'Video';

  @override
  String get feedUploading => 'Uploading media...';

  @override
  String get feedShareWorkout => 'Share Workout';

  @override
  String get feedShareWorkoutSummary => 'Just completed a workout!';

  @override
  String get sessionDetailExercises => 'Exercises';

  @override
  String get sessionDetailStartAgain => 'Start Again';

  @override
  String get sessionDetailSpeed => 'Speed';

  @override
  String get menuExercises => 'Exercises';

  @override
  String get labelFilterExercises => 'Filter Exercises';

  @override
  String get labelOwnerName => 'Owner Name';

  @override
  String get labelCategory => 'Category';

  @override
  String get labelVisibility => 'Visibility';

  @override
  String get labelSort => 'Sort By';

  @override
  String get buttonSaveAsWorkout => 'Save as Workout';

  @override
  String get buttonStartWorkout => 'Start Workout';

  @override
  String get buttonApplyFilters => 'Apply Filters';

  @override
  String get messageExercisesSelected => 'exercises selected';

  @override
  String get messageNoExercisesSelected => 'No exercises selected';

  @override
  String get messageNoExercisesFound => 'No exercises found';

  @override
  String get messageExercisesAvailable => 'Exercises';

  @override
  String get messageSelectAtLeastOne => 'Please select at least one exercise';

  @override
  String get messageStartWorkoutFeatureComing =>
      'Start workout feature coming soon';

  @override
  String get tooltipSwipeToView =>
      'Go back and swipe an exercise right to add it';

  @override
  String get tooltipBackToExercises => 'Back to exercises';

  @override
  String get sortCreatedAtDesc => 'Newest First';

  @override
  String get sortCreatedAtAsc => 'Oldest First';

  @override
  String get sortOwnerNameAsc => 'Owner Name (A-Z)';

  @override
  String get sortNameAsc => 'Exercise Name (A-Z)';

  @override
  String get labelSearchOwners => 'Filter by Owner';

  @override
  String get labelSelectedOwners => 'owners selected';

  @override
  String get messageSearchHint => 'Search people by name...';

  @override
  String get messageNoPersonsFound => 'No people found for that search.';

  @override
  String get messageUserDataNotLoaded =>
      'User data not loaded. Please try again.';

  @override
  String get messageWorkoutSavedSuccessfully => 'Workout saved successfully';

  @override
  String get tooltipAddExercise => 'Add Exercise';

  @override
  String get tooltipChangeLanguage => 'Change language';

  @override
  String get tooltipRefresh => 'Refresh';

  @override
  String get tooltipPreviousExercise => 'Previous exercise';

  @override
  String get labelSettings => 'Settings';

  @override
  String get messageAddToSelection => 'Add to selection';

  @override
  String get messageRemoveFromSelection => 'Remove';

  @override
  String get labelTakePhoto => 'Take Photo';

  @override
  String get labelRecordVideo => 'Record Video';

  @override
  String get labelVideo => 'VIDEO';

  @override
  String get buttonStartSession => 'Start Session';

  @override
  String get evolutionTitle => 'Evolution';

  @override
  String get evolutionFilterLastWeek => 'Last Week';

  @override
  String get evolutionFilterLastMonth => 'Last Month';

  @override
  String get evolutionFilterLast6Months => 'Last 6 Months';

  @override
  String get evolutionFilterCustom => 'Custom Range';

  @override
  String get evolutionCheckinsTitle => 'Check-ins';

  @override
  String get evolutionLatestMetrics => 'Latest Metrics';

  @override
  String get evolutionCompositionWeight => 'Weight';

  @override
  String get evolutionCompositionBodyFat => 'Body Fat';

  @override
  String get evolutionCompositionMuscleMass => 'Muscle Mass';

  @override
  String get evolutionCompositionVisceralFat => 'Visceral Fat';

  @override
  String get evolutionCircumferenceChest => 'Chest';

  @override
  String get evolutionCircumferenceWaist => 'Waist';

  @override
  String get evolutionCircumferenceHip => 'Hip';

  @override
  String get evolutionNoData => 'No evolution data for the selected period.';

  @override
  String get evolutionNoNote => 'No note';

  @override
  String get evolutionAddCheckin => 'Add Check-in';

  @override
  String get evolutionFormNote => 'Note';

  @override
  String get evolutionFormNotePlaceholder => 'e.g. Weekly update';

  @override
  String get evolutionFormVisibility => 'Visibility';

  @override
  String get evolutionSectionComposition => 'Body Composition';

  @override
  String get evolutionSectionCircumferences => 'Circumferences';

  @override
  String get evolutionCircumferenceNeck => 'Neck';

  @override
  String get evolutionCircumferenceAbdomen => 'Abdomen';

  @override
  String get evolutionCircumferenceBicepsRight => 'Biceps Right';

  @override
  String get evolutionCircumferenceBicepsLeft => 'Biceps Left';

  @override
  String get evolutionCircumferenceThighRight => 'Thigh Right';

  @override
  String get evolutionCircumferenceThighLeft => 'Thigh Left';

  @override
  String get evolutionAtLeastOneMetric =>
      'Add at least one metric before saving.';

  @override
  String get evolutionCreatedSuccessfully =>
      'Evolution check-in created successfully';

  @override
  String get evolutionVitruvianTitle => 'Body Progress (Vitruvian)';

  @override
  String get evolutionVitruvianLowChange => 'Low change';

  @override
  String get evolutionVitruvianHighChange => 'High change';

  @override
  String get evolutionVitruvianInterpolationNote =>
      'Some regions include interpolated values where check-ins had missing fields.';

  @override
  String get evolutionVitruvianBaselineProfileCheckin =>
      'baseline: profile + first check-in fallback';

  @override
  String get evolutionVitruvianBaselineCheckinOnly =>
      'baseline: first check-in only';

  @override
  String get evolutionVitruvianSetGenderHint =>
      'Set your gender in your profile for a more accurate model.';

  @override
  String get evolutionVitruvianShoulders => 'Shoulders';

  @override
  String get evolutionVitruvianArms => 'Arms';

  @override
  String get evolutionVitruvianAbdomen => 'Abdomen';

  @override
  String get evolutionVitruvianHips => 'Hips';

  @override
  String get evolutionVitruvianThighs => 'Thighs';

  @override
  String get evolutionVitruvianViewFront => 'Front';

  @override
  String get evolutionVitruvianViewBack => 'Back';

  @override
  String get evolutionVitruvianBack => 'Back';

  @override
  String get evolutionVitruvianUpperBack => 'Upper back';

  @override
  String get evolutionVitruvianTriceps => 'Triceps';

  @override
  String get evolutionVitruvianGlutes => 'Glutes';

  @override
  String get evolutionVitruvianCalves => 'Calves';

  @override
  String get buttonRetry => 'Retry';

  @override
  String exerciseCreateSuccess(String name) {
    return 'Exercise \"$name\" created successfully!';
  }

  @override
  String get exerciseUserDataUnavailable => 'User data not available';

  @override
  String get exerciseCreateFailed =>
      'Failed to create exercise. Please try again.';

  @override
  String get exerciseDurationMin => 'Duration (min)';

  @override
  String get addProfileTitle => 'Add a new profile';

  @override
  String get addProfileSubtitle =>
      'Choose the type of business profile you want to create';

  @override
  String get addProfilePersonalTrainerTitle => 'Personal Trainer';

  @override
  String get addProfilePersonalTrainerDescription =>
      'Offer personal training services and manage your own clients';

  @override
  String get addProfileGymTitle => 'Gym';

  @override
  String get addProfileGymDescription =>
      'Manage a gym or fitness facility and its members';

  @override
  String get businessProfileFormTitlePersonalTrainer =>
      'Create your Personal Trainer profile';

  @override
  String get businessProfileFormTitleGym => 'Create your Gym profile';

  @override
  String get businessProfileFormBusinessName => 'Business name';

  @override
  String get businessProfileFormBusinessNameRequired =>
      'Please enter a business name';

  @override
  String get businessProfileFormSocialName => 'Social name';

  @override
  String get businessProfileFormSocialNameRequired =>
      'Please enter a social name';

  @override
  String get businessProfileFormTaxId => 'Tax ID';

  @override
  String get businessProfileFormTaxIdRequired => 'Please enter a tax ID';

  @override
  String get businessProfileFormSubmit => 'Create profile';

  @override
  String get businessProfileFormError =>
      'Failed to create business profile. Please try again.';

  @override
  String get businessProfilePageEdit => 'Edit';

  @override
  String get businessProfilePageSave => 'Save';

  @override
  String get businessProfilePageAddresses => 'Addresses';

  @override
  String get businessProfileSwitchToPersonal => 'Personal account';

  @override
  String get consentPendingTitle => 'Legal consent required';

  @override
  String get consentPendingSubtitle =>
      'Please review and accept the documents below before you continue using the app.';

  @override
  String get consentReadDocument => 'Read';

  @override
  String get consentAgree => 'I agree';

  @override
  String get consentReviewAndAccept => 'Review and accept';

  @override
  String get consentAllAccepted => 'All set — thanks for accepting.';

  @override
  String get consentLogOut => 'Log out';

  @override
  String get consentClose => 'Close';

  @override
  String get consentLoading => 'Loading…';

  @override
  String get consentLoadFailed =>
      'Could not load your pending consents. Please try again.';

  @override
  String get consentNeverAccepted => 'You have not accepted this document yet.';

  @override
  String consentVersionOutdated(String version) {
    return 'A new version ($version) is available and must be accepted.';
  }

  @override
  String get chatConversationsTitle => 'Messages';

  @override
  String get chatEmpty => 'No conversations yet';

  @override
  String get chatMessageHint => 'Write a message…';

  @override
  String get chatSend => 'Send';

  @override
  String get chatYou => 'You';

  @override
  String get chatNotFriends => 'You can only message your friends or team.';

  @override
  String get chatTeamGroup => 'Team chat';

  @override
  String get chatBusinessDirect => 'Business chat';

  @override
  String get chatReconnecting => 'Reconnecting…';

  @override
  String get chatNewConversation => 'New conversation';

  @override
  String get chatStartConversationTitle => 'Start a conversation';

  @override
  String get chatOnline => 'Online';

  @override
  String get chatOffline => 'Offline';

  @override
  String get chatNoFriendsYet => 'Add friends to start a conversation';

  @override
  String get chatTyping => 'typing…';

  @override
  String get chatSent => 'Sent';

  @override
  String get chatRead => 'Read';

  @override
  String get chatFailedTap => 'Not sent — tap to retry';

  @override
  String get chatSearchHint => 'Search friends';

  @override
  String get chatMessageAction => 'Message';

  @override
  String get chatToday => 'Today';

  @override
  String get chatYesterday => 'Yesterday';
}
