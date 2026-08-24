import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:intl/intl.dart' as intl;

import 'app_localizations_en.dart';
import 'app_localizations_es.dart';
import 'app_localizations_fr.dart';
import 'app_localizations_nl.dart';
import 'app_localizations_pt.dart';

// ignore_for_file: type=lint

/// Callers can lookup localized strings with an instance of AppLocalizations
/// returned by `AppLocalizations.of(context)`.
///
/// Applications need to include `AppLocalizations.delegate()` in their app's
/// `localizationDelegates` list, and the locales they support in the app's
/// `supportedLocales` list. For example:
///
/// ```dart
/// import 'l10n/app_localizations.dart';
///
/// return MaterialApp(
///   localizationsDelegates: AppLocalizations.localizationsDelegates,
///   supportedLocales: AppLocalizations.supportedLocales,
///   home: MyApplicationHome(),
/// );
/// ```
///
/// ## Update pubspec.yaml
///
/// Please make sure to update your pubspec.yaml to include the following
/// packages:
///
/// ```yaml
/// dependencies:
///   # Internationalization support.
///   flutter_localizations:
///     sdk: flutter
///   intl: any # Use the pinned version from flutter_localizations
///
///   # Rest of dependencies
/// ```
///
/// ## iOS Applications
///
/// iOS applications define key application metadata, including supported
/// locales, in an Info.plist file that is built into the application bundle.
/// To configure the locales supported by your app, you’ll need to edit this
/// file.
///
/// First, open your project’s ios/Runner.xcworkspace Xcode workspace file.
/// Then, in the Project Navigator, open the Info.plist file under the Runner
/// project’s Runner folder.
///
/// Next, select the Information Property List item, select Add Item from the
/// Editor menu, then select Localizations from the pop-up menu.
///
/// Select and expand the newly-created Localizations item then, for each
/// locale your application supports, add a new item and select the locale
/// you wish to add from the pop-up menu in the Value field. This list should
/// be consistent with the languages listed in the AppLocalizations.supportedLocales
/// property.
abstract class AppLocalizations {
  AppLocalizations(String locale)
    : localeName = intl.Intl.canonicalizedLocale(locale.toString());

  final String localeName;

  static AppLocalizations? of(BuildContext context) {
    return Localizations.of<AppLocalizations>(context, AppLocalizations);
  }

  static const LocalizationsDelegate<AppLocalizations> delegate =
      _AppLocalizationsDelegate();

  /// A list of this localizations delegate along with the default localizations
  /// delegates.
  ///
  /// Returns a list of localizations delegates containing this delegate along with
  /// GlobalMaterialLocalizations.delegate, GlobalCupertinoLocalizations.delegate,
  /// and GlobalWidgetsLocalizations.delegate.
  ///
  /// Additional delegates can be added by appending to this list in
  /// MaterialApp. This list does not have to be used at all if a custom list
  /// of delegates is preferred or required.
  static const List<LocalizationsDelegate<dynamic>> localizationsDelegates =
      <LocalizationsDelegate<dynamic>>[
        delegate,
        GlobalMaterialLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
      ];

  /// A list of this localizations delegate's supported locales.
  static const List<Locale> supportedLocales = <Locale>[
    Locale('en'),
    Locale('es'),
    Locale('fr'),
    Locale('nl'),
    Locale('pt'),
    Locale('pt', 'BR'),
  ];

  /// No description provided for @appTitle.
  ///
  /// In en, this message translates to:
  /// **'Social Gym'**
  String get appTitle;

  /// No description provided for @signInTitle.
  ///
  /// In en, this message translates to:
  /// **'Log In'**
  String get signInTitle;

  /// No description provided for @signInEmail.
  ///
  /// In en, this message translates to:
  /// **'Email'**
  String get signInEmail;

  /// No description provided for @signInPassword.
  ///
  /// In en, this message translates to:
  /// **'Password'**
  String get signInPassword;

  /// No description provided for @signInSubmit.
  ///
  /// In en, this message translates to:
  /// **'Sign In'**
  String get signInSubmit;

  /// No description provided for @signInForgotPassword.
  ///
  /// In en, this message translates to:
  /// **'Forgot password?'**
  String get signInForgotPassword;

  /// No description provided for @signInCreateAccount.
  ///
  /// In en, this message translates to:
  /// **'Create new account'**
  String get signInCreateAccount;

  /// No description provided for @signInOr.
  ///
  /// In en, this message translates to:
  /// **'or'**
  String get signInOr;

  /// No description provided for @signUpTitle.
  ///
  /// In en, this message translates to:
  /// **'Create a new account'**
  String get signUpTitle;

  /// No description provided for @signUpFirstName.
  ///
  /// In en, this message translates to:
  /// **'First Name'**
  String get signUpFirstName;

  /// No description provided for @signUpSurname.
  ///
  /// In en, this message translates to:
  /// **'Surname'**
  String get signUpSurname;

  /// No description provided for @signUpBirthdayMonth.
  ///
  /// In en, this message translates to:
  /// **'Month'**
  String get signUpBirthdayMonth;

  /// No description provided for @signUpBirthdayDay.
  ///
  /// In en, this message translates to:
  /// **'Day'**
  String get signUpBirthdayDay;

  /// No description provided for @signUpBirthdayYear.
  ///
  /// In en, this message translates to:
  /// **'Year'**
  String get signUpBirthdayYear;

  /// No description provided for @signUpGender.
  ///
  /// In en, this message translates to:
  /// **'Gender'**
  String get signUpGender;

  /// No description provided for @signUpGenderFemale.
  ///
  /// In en, this message translates to:
  /// **'Female'**
  String get signUpGenderFemale;

  /// No description provided for @signUpGenderMale.
  ///
  /// In en, this message translates to:
  /// **'Male'**
  String get signUpGenderMale;

  /// No description provided for @signUpGenderCustom.
  ///
  /// In en, this message translates to:
  /// **'Custom'**
  String get signUpGenderCustom;

  /// No description provided for @signUpGenderCustomPlaceholder.
  ///
  /// In en, this message translates to:
  /// **'Please specify'**
  String get signUpGenderCustomPlaceholder;

  /// No description provided for @signUpMobileOrEmail.
  ///
  /// In en, this message translates to:
  /// **'Mobile number or email'**
  String get signUpMobileOrEmail;

  /// No description provided for @signUpPassword.
  ///
  /// In en, this message translates to:
  /// **'New password'**
  String get signUpPassword;

  /// No description provided for @signUpPasswordHint.
  ///
  /// In en, this message translates to:
  /// **'Use a combination of at least six numbers, letters and punctuation marks.'**
  String get signUpPasswordHint;

  /// No description provided for @signUpSubmit.
  ///
  /// In en, this message translates to:
  /// **'Sign Up'**
  String get signUpSubmit;

  /// No description provided for @signUpAlreadyHaveAccount.
  ///
  /// In en, this message translates to:
  /// **'Already have an account?'**
  String get signUpAlreadyHaveAccount;

  /// No description provided for @validationEmailRequired.
  ///
  /// In en, this message translates to:
  /// **'Please enter your email'**
  String get validationEmailRequired;

  /// No description provided for @validationPasswordRequired.
  ///
  /// In en, this message translates to:
  /// **'Please enter your password'**
  String get validationPasswordRequired;

  /// No description provided for @validationPasswordMinLength.
  ///
  /// In en, this message translates to:
  /// **'Password must be at least 6 characters'**
  String get validationPasswordMinLength;

  /// No description provided for @validationFirstNameRequired.
  ///
  /// In en, this message translates to:
  /// **'Please enter your first name'**
  String get validationFirstNameRequired;

  /// No description provided for @validationSurnameRequired.
  ///
  /// In en, this message translates to:
  /// **'Please enter your surname'**
  String get validationSurnameRequired;

  /// No description provided for @validationDateOfBirthRequired.
  ///
  /// In en, this message translates to:
  /// **'Please select your date of birth'**
  String get validationDateOfBirthRequired;

  /// No description provided for @validationGenderRequired.
  ///
  /// In en, this message translates to:
  /// **'Please select your gender'**
  String get validationGenderRequired;

  /// No description provided for @connectionError.
  ///
  /// In en, this message translates to:
  /// **'Connection error. Please try again.'**
  String get connectionError;

  /// No description provided for @homeWelcome.
  ///
  /// In en, this message translates to:
  /// **'Welcome to Social Gym!'**
  String get homeWelcome;

  /// No description provided for @homeTitle.
  ///
  /// In en, this message translates to:
  /// **'Social Gym'**
  String get homeTitle;

  /// No description provided for @logout.
  ///
  /// In en, this message translates to:
  /// **'Logout'**
  String get logout;

  /// No description provided for @monthJanuary.
  ///
  /// In en, this message translates to:
  /// **'January'**
  String get monthJanuary;

  /// No description provided for @monthFebruary.
  ///
  /// In en, this message translates to:
  /// **'February'**
  String get monthFebruary;

  /// No description provided for @monthMarch.
  ///
  /// In en, this message translates to:
  /// **'March'**
  String get monthMarch;

  /// No description provided for @monthApril.
  ///
  /// In en, this message translates to:
  /// **'April'**
  String get monthApril;

  /// No description provided for @monthMay.
  ///
  /// In en, this message translates to:
  /// **'May'**
  String get monthMay;

  /// No description provided for @monthJune.
  ///
  /// In en, this message translates to:
  /// **'June'**
  String get monthJune;

  /// No description provided for @monthJuly.
  ///
  /// In en, this message translates to:
  /// **'July'**
  String get monthJuly;

  /// No description provided for @monthAugust.
  ///
  /// In en, this message translates to:
  /// **'August'**
  String get monthAugust;

  /// No description provided for @monthSeptember.
  ///
  /// In en, this message translates to:
  /// **'September'**
  String get monthSeptember;

  /// No description provided for @monthOctober.
  ///
  /// In en, this message translates to:
  /// **'October'**
  String get monthOctober;

  /// No description provided for @monthNovember.
  ///
  /// In en, this message translates to:
  /// **'November'**
  String get monthNovember;

  /// No description provided for @monthDecember.
  ///
  /// In en, this message translates to:
  /// **'December'**
  String get monthDecember;

  /// No description provided for @menuOpen.
  ///
  /// In en, this message translates to:
  /// **'Open menu'**
  String get menuOpen;

  /// No description provided for @menuHome.
  ///
  /// In en, this message translates to:
  /// **'Home'**
  String get menuHome;

  /// No description provided for @menuFeed.
  ///
  /// In en, this message translates to:
  /// **'Feed'**
  String get menuFeed;

  /// No description provided for @menuGallery.
  ///
  /// In en, this message translates to:
  /// **'Gallery'**
  String get menuGallery;

  /// No description provided for @menuProfile.
  ///
  /// In en, this message translates to:
  /// **'Profile'**
  String get menuProfile;

  /// No description provided for @menuTeam.
  ///
  /// In en, this message translates to:
  /// **'Team'**
  String get menuTeam;

  /// No description provided for @menuFollowers.
  ///
  /// In en, this message translates to:
  /// **'Followers'**
  String get menuFollowers;

  /// No description provided for @menuFriends.
  ///
  /// In en, this message translates to:
  /// **'Friends'**
  String get menuFriends;

  /// No description provided for @menuMessages.
  ///
  /// In en, this message translates to:
  /// **'Messages'**
  String get menuMessages;

  /// No description provided for @menuNotifications.
  ///
  /// In en, this message translates to:
  /// **'Notifications'**
  String get menuNotifications;

  /// No description provided for @menuWorkouts.
  ///
  /// In en, this message translates to:
  /// **'Workouts'**
  String get menuWorkouts;

  /// No description provided for @menuWorkoutSessions.
  ///
  /// In en, this message translates to:
  /// **'Workout Sessions'**
  String get menuWorkoutSessions;

  /// No description provided for @menuEvolution.
  ///
  /// In en, this message translates to:
  /// **'Evolution'**
  String get menuEvolution;

  /// No description provided for @menuSettings.
  ///
  /// In en, this message translates to:
  /// **'Settings'**
  String get menuSettings;

  /// No description provided for @menuLanguage.
  ///
  /// In en, this message translates to:
  /// **'Language'**
  String get menuLanguage;

  /// No description provided for @menuCollapse.
  ///
  /// In en, this message translates to:
  /// **'Collapse'**
  String get menuCollapse;

  /// No description provided for @menuOnline.
  ///
  /// In en, this message translates to:
  /// **'Online'**
  String get menuOnline;

  /// No description provided for @menuOffline.
  ///
  /// In en, this message translates to:
  /// **'Offline'**
  String get menuOffline;

  /// No description provided for @searchPlaceholder.
  ///
  /// In en, this message translates to:
  /// **'Search Social Gym'**
  String get searchPlaceholder;

  /// No description provided for @comingSoon.
  ///
  /// In en, this message translates to:
  /// **'Coming soon'**
  String get comingSoon;

  /// No description provided for @buttonCancel.
  ///
  /// In en, this message translates to:
  /// **'Cancel'**
  String get buttonCancel;

  /// No description provided for @buttonConfirm.
  ///
  /// In en, this message translates to:
  /// **'Confirm'**
  String get buttonConfirm;

  /// No description provided for @buttonSave.
  ///
  /// In en, this message translates to:
  /// **'Save'**
  String get buttonSave;

  /// No description provided for @validationRequired.
  ///
  /// In en, this message translates to:
  /// **'This field is required'**
  String get validationRequired;

  /// No description provided for @permissionRequired.
  ///
  /// In en, this message translates to:
  /// **'Permission Required'**
  String get permissionRequired;

  /// No description provided for @cameraPermissionDenied.
  ///
  /// In en, this message translates to:
  /// **'Camera access is required to take photos. Please enable it in your device settings.'**
  String get cameraPermissionDenied;

  /// No description provided for @photosPermissionDenied.
  ///
  /// In en, this message translates to:
  /// **'Photo library access is required to select images. Please enable it in your device settings.'**
  String get photosPermissionDenied;

  /// No description provided for @openSettings.
  ///
  /// In en, this message translates to:
  /// **'Open Settings'**
  String get openSettings;

  /// No description provided for @settingsTitle.
  ///
  /// In en, this message translates to:
  /// **'Settings'**
  String get settingsTitle;

  /// No description provided for @settingsLanguage.
  ///
  /// In en, this message translates to:
  /// **'Language'**
  String get settingsLanguage;

  /// No description provided for @settingsNotificationsEnabled.
  ///
  /// In en, this message translates to:
  /// **'Enable notifications'**
  String get settingsNotificationsEnabled;

  /// No description provided for @settingsContextMenuPosition.
  ///
  /// In en, this message translates to:
  /// **'Sidebar menu position'**
  String get settingsContextMenuPosition;

  /// No description provided for @settingsContextMenuPositionLeft.
  ///
  /// In en, this message translates to:
  /// **'Left'**
  String get settingsContextMenuPositionLeft;

  /// No description provided for @settingsContextMenuPositionTop.
  ///
  /// In en, this message translates to:
  /// **'Top'**
  String get settingsContextMenuPositionTop;

  /// No description provided for @settingsContextMenuPositionRight.
  ///
  /// In en, this message translates to:
  /// **'Right'**
  String get settingsContextMenuPositionRight;

  /// No description provided for @settingsContextMenuPositionBottom.
  ///
  /// In en, this message translates to:
  /// **'Bottom'**
  String get settingsContextMenuPositionBottom;

  /// No description provided for @settingsHomePage.
  ///
  /// In en, this message translates to:
  /// **'Home page'**
  String get settingsHomePage;

  /// No description provided for @settingsHomePageFeed.
  ///
  /// In en, this message translates to:
  /// **'Feed'**
  String get settingsHomePageFeed;

  /// No description provided for @settingsHomePageGallery.
  ///
  /// In en, this message translates to:
  /// **'Gallery'**
  String get settingsHomePageGallery;

  /// No description provided for @settingsSaveSuccess.
  ///
  /// In en, this message translates to:
  /// **'Settings saved'**
  String get settingsSaveSuccess;

  /// No description provided for @settingsSaveError.
  ///
  /// In en, this message translates to:
  /// **'Could not save settings'**
  String get settingsSaveError;

  /// No description provided for @settingsLoadError.
  ///
  /// In en, this message translates to:
  /// **'Could not refresh settings from the server'**
  String get settingsLoadError;

  /// No description provided for @workoutTitle.
  ///
  /// In en, this message translates to:
  /// **'My Workouts'**
  String get workoutTitle;

  /// No description provided for @workoutDescription.
  ///
  /// In en, this message translates to:
  /// **'Manage your workout routines and exercises'**
  String get workoutDescription;

  /// No description provided for @workoutAddNew.
  ///
  /// In en, this message translates to:
  /// **'Add Workout'**
  String get workoutAddNew;

  /// No description provided for @workoutEditTitle.
  ///
  /// In en, this message translates to:
  /// **'Edit Workout'**
  String get workoutEditTitle;

  /// No description provided for @workoutStartSessionTitle.
  ///
  /// In en, this message translates to:
  /// **'Start Workout Session'**
  String get workoutStartSessionTitle;

  /// No description provided for @workoutDeleteConfirmTitle.
  ///
  /// In en, this message translates to:
  /// **'Delete Workout'**
  String get workoutDeleteConfirmTitle;

  /// No description provided for @workoutDeleteConfirm.
  ///
  /// In en, this message translates to:
  /// **'Are you sure you want to delete this workout?'**
  String get workoutDeleteConfirm;

  /// No description provided for @workoutExercises.
  ///
  /// In en, this message translates to:
  /// **'exercises'**
  String get workoutExercises;

  /// No description provided for @workoutExercisesFor.
  ///
  /// In en, this message translates to:
  /// **'Exercises for'**
  String get workoutExercisesFor;

  /// No description provided for @workoutNoWorkouts.
  ///
  /// In en, this message translates to:
  /// **'No workouts yet. Create your first one!'**
  String get workoutNoWorkouts;

  /// No description provided for @workoutNoExercises.
  ///
  /// In en, this message translates to:
  /// **'No exercises yet'**
  String get workoutNoExercises;

  /// No description provided for @workoutAddExercise.
  ///
  /// In en, this message translates to:
  /// **'Add Exercise'**
  String get workoutAddExercise;

  /// No description provided for @workoutAddExerciseHint.
  ///
  /// In en, this message translates to:
  /// **'Add exercises to start training'**
  String get workoutAddExerciseHint;

  /// No description provided for @workoutFormName.
  ///
  /// In en, this message translates to:
  /// **'Name'**
  String get workoutFormName;

  /// No description provided for @workoutFormNamePlaceholder.
  ///
  /// In en, this message translates to:
  /// **'Workout name'**
  String get workoutFormNamePlaceholder;

  /// No description provided for @workoutFormDescription.
  ///
  /// In en, this message translates to:
  /// **'Description'**
  String get workoutFormDescription;

  /// No description provided for @workoutFormDescriptionPlaceholder.
  ///
  /// In en, this message translates to:
  /// **'Brief description'**
  String get workoutFormDescriptionPlaceholder;

  /// No description provided for @workoutDifficulty.
  ///
  /// In en, this message translates to:
  /// **'Difficulty'**
  String get workoutDifficulty;

  /// No description provided for @workoutVisibility.
  ///
  /// In en, this message translates to:
  /// **'Visibility'**
  String get workoutVisibility;

  /// No description provided for @workoutMuscleGroups.
  ///
  /// In en, this message translates to:
  /// **'Muscle Groups'**
  String get workoutMuscleGroups;

  /// No description provided for @workoutAssignToTeamMember.
  ///
  /// In en, this message translates to:
  /// **'Create for'**
  String get workoutAssignToTeamMember;

  /// No description provided for @workoutAssignToMyself.
  ///
  /// In en, this message translates to:
  /// **'Myself'**
  String get workoutAssignToMyself;

  /// No description provided for @workoutSelectTeamMember.
  ///
  /// In en, this message translates to:
  /// **'Select team member'**
  String get workoutSelectTeamMember;

  /// No description provided for @workoutNoTeamMembers.
  ///
  /// In en, this message translates to:
  /// **'You have no accepted team members yet'**
  String get workoutNoTeamMembers;

  /// No description provided for @workoutSets.
  ///
  /// In en, this message translates to:
  /// **'Sets'**
  String get workoutSets;

  /// No description provided for @workoutReps.
  ///
  /// In en, this message translates to:
  /// **'Reps'**
  String get workoutReps;

  /// No description provided for @workoutWeight.
  ///
  /// In en, this message translates to:
  /// **'Weight'**
  String get workoutWeight;

  /// No description provided for @workoutWeightUnit.
  ///
  /// In en, this message translates to:
  /// **'kg'**
  String get workoutWeightUnit;

  /// No description provided for @workoutExerciseName.
  ///
  /// In en, this message translates to:
  /// **'Exercise Name'**
  String get workoutExerciseName;

  /// No description provided for @workoutExerciseNamePlaceholder.
  ///
  /// In en, this message translates to:
  /// **'e.g. Bench Press'**
  String get workoutExerciseNamePlaceholder;

  /// No description provided for @workoutExerciseDescription.
  ///
  /// In en, this message translates to:
  /// **'Description'**
  String get workoutExerciseDescription;

  /// No description provided for @workoutExerciseDescriptionPlaceholder.
  ///
  /// In en, this message translates to:
  /// **'Exercise description'**
  String get workoutExerciseDescriptionPlaceholder;

  /// No description provided for @workoutExercisesList.
  ///
  /// In en, this message translates to:
  /// **'Exercises to add'**
  String get workoutExercisesList;

  /// No description provided for @difficultySoft.
  ///
  /// In en, this message translates to:
  /// **'Soft'**
  String get difficultySoft;

  /// No description provided for @difficultyEasy.
  ///
  /// In en, this message translates to:
  /// **'Easy'**
  String get difficultyEasy;

  /// No description provided for @difficultyMedium.
  ///
  /// In en, this message translates to:
  /// **'Medium'**
  String get difficultyMedium;

  /// No description provided for @difficultyHard.
  ///
  /// In en, this message translates to:
  /// **'Hard'**
  String get difficultyHard;

  /// No description provided for @difficultyStrong.
  ///
  /// In en, this message translates to:
  /// **'Strong'**
  String get difficultyStrong;

  /// No description provided for @visibilityPrivate.
  ///
  /// In en, this message translates to:
  /// **'Private'**
  String get visibilityPrivate;

  /// No description provided for @visibilityFriends.
  ///
  /// In en, this message translates to:
  /// **'Friends'**
  String get visibilityFriends;

  /// No description provided for @visibilityPublic.
  ///
  /// In en, this message translates to:
  /// **'Public'**
  String get visibilityPublic;

  /// No description provided for @visibilityProfessional.
  ///
  /// In en, this message translates to:
  /// **'Professional'**
  String get visibilityProfessional;

  /// No description provided for @muscleGroupChest.
  ///
  /// In en, this message translates to:
  /// **'Chest'**
  String get muscleGroupChest;

  /// No description provided for @muscleGroupLegs.
  ///
  /// In en, this message translates to:
  /// **'Legs'**
  String get muscleGroupLegs;

  /// No description provided for @muscleGroupBack.
  ///
  /// In en, this message translates to:
  /// **'Back'**
  String get muscleGroupBack;

  /// No description provided for @muscleGroupCore.
  ///
  /// In en, this message translates to:
  /// **'Core'**
  String get muscleGroupCore;

  /// No description provided for @muscleGroupFullBody.
  ///
  /// In en, this message translates to:
  /// **'Full Body'**
  String get muscleGroupFullBody;

  /// No description provided for @muscleGroupShoulders.
  ///
  /// In en, this message translates to:
  /// **'Shoulders'**
  String get muscleGroupShoulders;

  /// No description provided for @muscleGroupArms.
  ///
  /// In en, this message translates to:
  /// **'Arms'**
  String get muscleGroupArms;

  /// No description provided for @muscleGroupGlutes.
  ///
  /// In en, this message translates to:
  /// **'Glutes'**
  String get muscleGroupGlutes;

  /// No description provided for @executionStartWorkout.
  ///
  /// In en, this message translates to:
  /// **'Start Workout'**
  String get executionStartWorkout;

  /// No description provided for @executionExercise.
  ///
  /// In en, this message translates to:
  /// **'Exercise'**
  String get executionExercise;

  /// No description provided for @executionSet.
  ///
  /// In en, this message translates to:
  /// **'Set'**
  String get executionSet;

  /// No description provided for @executionOf.
  ///
  /// In en, this message translates to:
  /// **'of'**
  String get executionOf;

  /// No description provided for @executionConfirmSet.
  ///
  /// In en, this message translates to:
  /// **'Confirm Set'**
  String get executionConfirmSet;

  /// No description provided for @executionSkip.
  ///
  /// In en, this message translates to:
  /// **'Skip'**
  String get executionSkip;

  /// No description provided for @executionKeepGoing.
  ///
  /// In en, this message translates to:
  /// **'Keep going! You\'re doing great!'**
  String get executionKeepGoing;

  /// No description provided for @executionAlmostDone.
  ///
  /// In en, this message translates to:
  /// **'Almost done! Last effort!'**
  String get executionAlmostDone;

  /// No description provided for @executionCompletedSets.
  ///
  /// In en, this message translates to:
  /// **'Completed Sets'**
  String get executionCompletedSets;

  /// No description provided for @executionWorkoutComplete.
  ///
  /// In en, this message translates to:
  /// **'Workout Complete!'**
  String get executionWorkoutComplete;

  /// No description provided for @executionDuration.
  ///
  /// In en, this message translates to:
  /// **'Duration'**
  String get executionDuration;

  /// No description provided for @executionSetsCompleted.
  ///
  /// In en, this message translates to:
  /// **'Sets Completed'**
  String get executionSetsCompleted;

  /// No description provided for @executionTotalVolume.
  ///
  /// In en, this message translates to:
  /// **'Total Volume'**
  String get executionTotalVolume;

  /// No description provided for @executionGreatJob.
  ///
  /// In en, this message translates to:
  /// **'Great job! You crushed this workout! 💪'**
  String get executionGreatJob;

  /// No description provided for @executionClose.
  ///
  /// In en, this message translates to:
  /// **'Close'**
  String get executionClose;

  /// No description provided for @executionSaving.
  ///
  /// In en, this message translates to:
  /// **'Saving...'**
  String get executionSaving;

  /// No description provided for @executionSaveSession.
  ///
  /// In en, this message translates to:
  /// **'Save Session'**
  String get executionSaveSession;

  /// No description provided for @executionQuitTitle.
  ///
  /// In en, this message translates to:
  /// **'Quit Workout?'**
  String get executionQuitTitle;

  /// No description provided for @executionQuitMessage.
  ///
  /// In en, this message translates to:
  /// **'Your progress will be lost. Are you sure?'**
  String get executionQuitMessage;

  /// No description provided for @executionQuitConfirm.
  ///
  /// In en, this message translates to:
  /// **'Quit'**
  String get executionQuitConfirm;

  /// No description provided for @profileTitle.
  ///
  /// In en, this message translates to:
  /// **'Profile'**
  String get profileTitle;

  /// No description provided for @profileEditProfile.
  ///
  /// In en, this message translates to:
  /// **'Edit Profile'**
  String get profileEditProfile;

  /// No description provided for @profileChangeCover.
  ///
  /// In en, this message translates to:
  /// **'Change Cover'**
  String get profileChangeCover;

  /// No description provided for @profileChangeAvatar.
  ///
  /// In en, this message translates to:
  /// **'Change Avatar'**
  String get profileChangeAvatar;

  /// No description provided for @profilePersonalInfo.
  ///
  /// In en, this message translates to:
  /// **'Personal Information'**
  String get profilePersonalInfo;

  /// No description provided for @profileFirstName.
  ///
  /// In en, this message translates to:
  /// **'First Name'**
  String get profileFirstName;

  /// No description provided for @profileSurname.
  ///
  /// In en, this message translates to:
  /// **'Surname'**
  String get profileSurname;

  /// No description provided for @profileDateOfBirth.
  ///
  /// In en, this message translates to:
  /// **'Date of Birth'**
  String get profileDateOfBirth;

  /// No description provided for @profileGender.
  ///
  /// In en, this message translates to:
  /// **'Gender'**
  String get profileGender;

  /// No description provided for @profileBiography.
  ///
  /// In en, this message translates to:
  /// **'Biography'**
  String get profileBiography;

  /// No description provided for @profileBiographyHint.
  ///
  /// In en, this message translates to:
  /// **'Tell us about yourself...'**
  String get profileBiographyHint;

  /// No description provided for @profileJob.
  ///
  /// In en, this message translates to:
  /// **'Job'**
  String get profileJob;

  /// No description provided for @profileJobHint.
  ///
  /// In en, this message translates to:
  /// **'What do you do?'**
  String get profileJobHint;

  /// No description provided for @profileRelationship.
  ///
  /// In en, this message translates to:
  /// **'Relationship Status'**
  String get profileRelationship;

  /// No description provided for @profileHomeTown.
  ///
  /// In en, this message translates to:
  /// **'Hometown'**
  String get profileHomeTown;

  /// No description provided for @profileCurrentCity.
  ///
  /// In en, this message translates to:
  /// **'Current City'**
  String get profileCurrentCity;

  /// No description provided for @profilePhysicalStats.
  ///
  /// In en, this message translates to:
  /// **'Physical Stats'**
  String get profilePhysicalStats;

  /// No description provided for @profileWeight.
  ///
  /// In en, this message translates to:
  /// **'Weight (kg)'**
  String get profileWeight;

  /// No description provided for @profileHeight.
  ///
  /// In en, this message translates to:
  /// **'Height (cm)'**
  String get profileHeight;

  /// No description provided for @profileSaveChanges.
  ///
  /// In en, this message translates to:
  /// **'Save Changes'**
  String get profileSaveChanges;

  /// No description provided for @profileUpdateSuccess.
  ///
  /// In en, this message translates to:
  /// **'Profile updated successfully'**
  String get profileUpdateSuccess;

  /// No description provided for @profileUpdateError.
  ///
  /// In en, this message translates to:
  /// **'Failed to update profile'**
  String get profileUpdateError;

  /// No description provided for @profileUploadingImage.
  ///
  /// In en, this message translates to:
  /// **'Uploading image...'**
  String get profileUploadingImage;

  /// No description provided for @profileImageUploadSuccess.
  ///
  /// In en, this message translates to:
  /// **'Image uploaded successfully'**
  String get profileImageUploadSuccess;

  /// No description provided for @profileImageUploadError.
  ///
  /// In en, this message translates to:
  /// **'Failed to upload image'**
  String get profileImageUploadError;

  /// No description provided for @profileSelectImage.
  ///
  /// In en, this message translates to:
  /// **'Select Image'**
  String get profileSelectImage;

  /// No description provided for @profileTakePhoto.
  ///
  /// In en, this message translates to:
  /// **'Take Photo'**
  String get profileTakePhoto;

  /// No description provided for @profileChooseFromGallery.
  ///
  /// In en, this message translates to:
  /// **'Choose from Gallery'**
  String get profileChooseFromGallery;

  /// No description provided for @profileRemovePhoto.
  ///
  /// In en, this message translates to:
  /// **'Remove Photo'**
  String get profileRemovePhoto;

  /// No description provided for @profileGenderMale.
  ///
  /// In en, this message translates to:
  /// **'Male'**
  String get profileGenderMale;

  /// No description provided for @profileGenderFemale.
  ///
  /// In en, this message translates to:
  /// **'Female'**
  String get profileGenderFemale;

  /// No description provided for @profileGenderOther.
  ///
  /// In en, this message translates to:
  /// **'Other'**
  String get profileGenderOther;

  /// No description provided for @profileRelationshipSingle.
  ///
  /// In en, this message translates to:
  /// **'Single'**
  String get profileRelationshipSingle;

  /// No description provided for @profileRelationshipInRelationship.
  ///
  /// In en, this message translates to:
  /// **'In a Relationship'**
  String get profileRelationshipInRelationship;

  /// No description provided for @profileRelationshipEngaged.
  ///
  /// In en, this message translates to:
  /// **'Engaged'**
  String get profileRelationshipEngaged;

  /// No description provided for @profileRelationshipMarried.
  ///
  /// In en, this message translates to:
  /// **'Married'**
  String get profileRelationshipMarried;

  /// No description provided for @profileRelationshipComplicated.
  ///
  /// In en, this message translates to:
  /// **'It\'s Complicated'**
  String get profileRelationshipComplicated;

  /// No description provided for @profileMemberSince.
  ///
  /// In en, this message translates to:
  /// **'Member since'**
  String get profileMemberSince;

  /// No description provided for @addressSection.
  ///
  /// In en, this message translates to:
  /// **'Addresses'**
  String get addressSection;

  /// No description provided for @addressAdd.
  ///
  /// In en, this message translates to:
  /// **'Add Address'**
  String get addressAdd;

  /// No description provided for @addressEdit.
  ///
  /// In en, this message translates to:
  /// **'Edit Address'**
  String get addressEdit;

  /// No description provided for @addressDelete.
  ///
  /// In en, this message translates to:
  /// **'Delete Address'**
  String get addressDelete;

  /// No description provided for @addressSetCurrent.
  ///
  /// In en, this message translates to:
  /// **'Set as Current'**
  String get addressSetCurrent;

  /// No description provided for @addressCurrent.
  ///
  /// In en, this message translates to:
  /// **'Current Address'**
  String get addressCurrent;

  /// No description provided for @addressStreet.
  ///
  /// In en, this message translates to:
  /// **'Street'**
  String get addressStreet;

  /// No description provided for @addressNumber.
  ///
  /// In en, this message translates to:
  /// **'Number'**
  String get addressNumber;

  /// No description provided for @addressComplement.
  ///
  /// In en, this message translates to:
  /// **'Complement'**
  String get addressComplement;

  /// No description provided for @addressNeighborhood.
  ///
  /// In en, this message translates to:
  /// **'Neighborhood'**
  String get addressNeighborhood;

  /// No description provided for @addressCity.
  ///
  /// In en, this message translates to:
  /// **'City'**
  String get addressCity;

  /// No description provided for @addressState.
  ///
  /// In en, this message translates to:
  /// **'State/Province'**
  String get addressState;

  /// No description provided for @addressCountry.
  ///
  /// In en, this message translates to:
  /// **'Country'**
  String get addressCountry;

  /// No description provided for @addressZipCode.
  ///
  /// In en, this message translates to:
  /// **'ZIP Code'**
  String get addressZipCode;

  /// No description provided for @addressPostalCode.
  ///
  /// In en, this message translates to:
  /// **'Postal Code'**
  String get addressPostalCode;

  /// No description provided for @addressLocality.
  ///
  /// In en, this message translates to:
  /// **'City'**
  String get addressLocality;

  /// No description provided for @addressLine1.
  ///
  /// In en, this message translates to:
  /// **'Address Line 1'**
  String get addressLine1;

  /// No description provided for @addressLine2.
  ///
  /// In en, this message translates to:
  /// **'Address Line 2'**
  String get addressLine2;

  /// No description provided for @addressAdministrativeArea.
  ///
  /// In en, this message translates to:
  /// **'State/Province'**
  String get addressAdministrativeArea;

  /// No description provided for @addressMarkCurrent.
  ///
  /// In en, this message translates to:
  /// **'Mark as current address'**
  String get addressMarkCurrent;

  /// No description provided for @addressSelectCountry.
  ///
  /// In en, this message translates to:
  /// **'Select country'**
  String get addressSelectCountry;

  /// No description provided for @addressSelectState.
  ///
  /// In en, this message translates to:
  /// **'Select state'**
  String get addressSelectState;

  /// No description provided for @addressSelectCountryFirst.
  ///
  /// In en, this message translates to:
  /// **'Select country first'**
  String get addressSelectCountryFirst;

  /// No description provided for @addressLatitude.
  ///
  /// In en, this message translates to:
  /// **'Latitude'**
  String get addressLatitude;

  /// No description provided for @addressLongitude.
  ///
  /// In en, this message translates to:
  /// **'Longitude'**
  String get addressLongitude;

  /// No description provided for @addressGpsLocation.
  ///
  /// In en, this message translates to:
  /// **'GPS Location'**
  String get addressGpsLocation;

  /// No description provided for @addressGetCurrentLocation.
  ///
  /// In en, this message translates to:
  /// **'Get Current Location'**
  String get addressGetCurrentLocation;

  /// No description provided for @addressLocationServicesDisabled.
  ///
  /// In en, this message translates to:
  /// **'Location services are disabled. Please enable them.'**
  String get addressLocationServicesDisabled;

  /// No description provided for @addressLocationPermissionDenied.
  ///
  /// In en, this message translates to:
  /// **'Location permission denied.'**
  String get addressLocationPermissionDenied;

  /// No description provided for @addressLocationPermissionDeniedForever.
  ///
  /// In en, this message translates to:
  /// **'Location permissions are permanently denied. Please enable them in your device settings.'**
  String get addressLocationPermissionDeniedForever;

  /// No description provided for @addressLocationError.
  ///
  /// In en, this message translates to:
  /// **'Failed to get current location.'**
  String get addressLocationError;

  /// No description provided for @addressNoAddresses.
  ///
  /// In en, this message translates to:
  /// **'No addresses added'**
  String get addressNoAddresses;

  /// No description provided for @addressNoAddressesHint.
  ///
  /// In en, this message translates to:
  /// **'Add your first address'**
  String get addressNoAddressesHint;

  /// No description provided for @addressAddSuccess.
  ///
  /// In en, this message translates to:
  /// **'Address added successfully'**
  String get addressAddSuccess;

  /// No description provided for @addressUpdateSuccess.
  ///
  /// In en, this message translates to:
  /// **'Address updated successfully'**
  String get addressUpdateSuccess;

  /// No description provided for @addressDeleteSuccess.
  ///
  /// In en, this message translates to:
  /// **'Address deleted successfully'**
  String get addressDeleteSuccess;

  /// No description provided for @addressSetCurrentSuccess.
  ///
  /// In en, this message translates to:
  /// **'Address set as current'**
  String get addressSetCurrentSuccess;

  /// No description provided for @addressActionError.
  ///
  /// In en, this message translates to:
  /// **'Action failed. Please try again.'**
  String get addressActionError;

  /// No description provided for @addressDeleteConfirm.
  ///
  /// In en, this message translates to:
  /// **'Are you sure you want to delete this address?'**
  String get addressDeleteConfirm;

  /// No description provided for @addressDeleteConfirmTitle.
  ///
  /// In en, this message translates to:
  /// **'Delete Address'**
  String get addressDeleteConfirmTitle;

  /// No description provided for @friendsTitle.
  ///
  /// In en, this message translates to:
  /// **'Friends'**
  String get friendsTitle;

  /// No description provided for @friendsDescription.
  ///
  /// In en, this message translates to:
  /// **'Connect with other gym enthusiasts'**
  String get friendsDescription;

  /// No description provided for @friendsTabAll.
  ///
  /// In en, this message translates to:
  /// **'Friends'**
  String get friendsTabAll;

  /// No description provided for @friendsTabRequests.
  ///
  /// In en, this message translates to:
  /// **'Requests'**
  String get friendsTabRequests;

  /// No description provided for @friendsTabSuggestions.
  ///
  /// In en, this message translates to:
  /// **'Suggestions'**
  String get friendsTabSuggestions;

  /// No description provided for @friendsNoFriends.
  ///
  /// In en, this message translates to:
  /// **'No friends yet'**
  String get friendsNoFriends;

  /// No description provided for @friendsNoFriendsHint.
  ///
  /// In en, this message translates to:
  /// **'Start connecting with other gym enthusiasts'**
  String get friendsNoFriendsHint;

  /// No description provided for @friendsNoRequests.
  ///
  /// In en, this message translates to:
  /// **'No friend requests'**
  String get friendsNoRequests;

  /// No description provided for @friendsNoRequestsHint.
  ///
  /// In en, this message translates to:
  /// **'You don\'t have any pending requests'**
  String get friendsNoRequestsHint;

  /// No description provided for @friendsNoSuggestions.
  ///
  /// In en, this message translates to:
  /// **'No suggestions available'**
  String get friendsNoSuggestions;

  /// No description provided for @friendsNoSuggestionsHint.
  ///
  /// In en, this message translates to:
  /// **'Check back later for new suggestions'**
  String get friendsNoSuggestionsHint;

  /// No description provided for @friendsAddFriend.
  ///
  /// In en, this message translates to:
  /// **'Add Friend'**
  String get friendsAddFriend;

  /// No description provided for @friendsAccept.
  ///
  /// In en, this message translates to:
  /// **'Accept'**
  String get friendsAccept;

  /// No description provided for @friendsReject.
  ///
  /// In en, this message translates to:
  /// **'Reject'**
  String get friendsReject;

  /// No description provided for @friendsCancel.
  ///
  /// In en, this message translates to:
  /// **'Cancel Request'**
  String get friendsCancel;

  /// No description provided for @friendsRemove.
  ///
  /// In en, this message translates to:
  /// **'Remove Friend'**
  String get friendsRemove;

  /// No description provided for @friendsPending.
  ///
  /// In en, this message translates to:
  /// **'Pending'**
  String get friendsPending;

  /// No description provided for @friendsReceivedRequests.
  ///
  /// In en, this message translates to:
  /// **'Received Requests'**
  String get friendsReceivedRequests;

  /// No description provided for @friendsSentRequests.
  ///
  /// In en, this message translates to:
  /// **'Sent Requests'**
  String get friendsSentRequests;

  /// No description provided for @friendsRequestSent.
  ///
  /// In en, this message translates to:
  /// **'Friend request sent'**
  String get friendsRequestSent;

  /// No description provided for @friendsRequestAccepted.
  ///
  /// In en, this message translates to:
  /// **'Friend request accepted'**
  String get friendsRequestAccepted;

  /// No description provided for @friendsRequestRejected.
  ///
  /// In en, this message translates to:
  /// **'Friend request rejected'**
  String get friendsRequestRejected;

  /// No description provided for @friendsRequestCancelled.
  ///
  /// In en, this message translates to:
  /// **'Friend request cancelled'**
  String get friendsRequestCancelled;

  /// No description provided for @friendsRemoved.
  ///
  /// In en, this message translates to:
  /// **'Friend removed'**
  String get friendsRemoved;

  /// No description provided for @friendsActionError.
  ///
  /// In en, this message translates to:
  /// **'Action failed. Please try again.'**
  String get friendsActionError;

  /// No description provided for @friendViewProfile.
  ///
  /// In en, this message translates to:
  /// **'View Profile'**
  String get friendViewProfile;

  /// No description provided for @profileFriendLoadError.
  ///
  /// In en, this message translates to:
  /// **'Could not load full profile. Basic info is shown.'**
  String get profileFriendLoadError;

  /// No description provided for @teamTitle.
  ///
  /// In en, this message translates to:
  /// **'Team'**
  String get teamTitle;

  /// No description provided for @teamDescription.
  ///
  /// In en, this message translates to:
  /// **'Manage your business team and memberships'**
  String get teamDescription;

  /// No description provided for @teamTabMembers.
  ///
  /// In en, this message translates to:
  /// **'Members'**
  String get teamTabMembers;

  /// No description provided for @teamTabSentInvites.
  ///
  /// In en, this message translates to:
  /// **'Sent Invites'**
  String get teamTabSentInvites;

  /// No description provided for @teamTabMyTeams.
  ///
  /// In en, this message translates to:
  /// **'My Teams'**
  String get teamTabMyTeams;

  /// No description provided for @teamTabReceivedInvites.
  ///
  /// In en, this message translates to:
  /// **'Received Invites'**
  String get teamTabReceivedInvites;

  /// No description provided for @teamNoMembers.
  ///
  /// In en, this message translates to:
  /// **'No team members yet'**
  String get teamNoMembers;

  /// No description provided for @teamNoMembersHint.
  ///
  /// In en, this message translates to:
  /// **'Invite people to join your team'**
  String get teamNoMembersHint;

  /// No description provided for @teamNoSentInvites.
  ///
  /// In en, this message translates to:
  /// **'No pending invites'**
  String get teamNoSentInvites;

  /// No description provided for @teamNoSentInvitesHint.
  ///
  /// In en, this message translates to:
  /// **'Invites you send will appear here'**
  String get teamNoSentInvitesHint;

  /// No description provided for @teamNoTeams.
  ///
  /// In en, this message translates to:
  /// **'You haven\'t joined any team'**
  String get teamNoTeams;

  /// No description provided for @teamNoTeamsHint.
  ///
  /// In en, this message translates to:
  /// **'Businesses you join will appear here'**
  String get teamNoTeamsHint;

  /// No description provided for @teamNoReceivedInvites.
  ///
  /// In en, this message translates to:
  /// **'No invites received'**
  String get teamNoReceivedInvites;

  /// No description provided for @teamNoReceivedInvitesHint.
  ///
  /// In en, this message translates to:
  /// **'Invitations from businesses will appear here'**
  String get teamNoReceivedInvitesHint;

  /// No description provided for @teamInvite.
  ///
  /// In en, this message translates to:
  /// **'Invite'**
  String get teamInvite;

  /// No description provided for @teamInviteSearchHint.
  ///
  /// In en, this message translates to:
  /// **'Search by name or email'**
  String get teamInviteSearchHint;

  /// No description provided for @teamInviteSend.
  ///
  /// In en, this message translates to:
  /// **'Send Invite'**
  String get teamInviteSend;

  /// No description provided for @teamInviteNoResults.
  ///
  /// In en, this message translates to:
  /// **'No people found'**
  String get teamInviteNoResults;

  /// No description provided for @teamAccept.
  ///
  /// In en, this message translates to:
  /// **'Accept'**
  String get teamAccept;

  /// No description provided for @teamDeny.
  ///
  /// In en, this message translates to:
  /// **'Deny'**
  String get teamDeny;

  /// No description provided for @teamCancel.
  ///
  /// In en, this message translates to:
  /// **'Cancel'**
  String get teamCancel;

  /// No description provided for @teamRequestSent.
  ///
  /// In en, this message translates to:
  /// **'Invite sent'**
  String get teamRequestSent;

  /// No description provided for @teamRequestAccepted.
  ///
  /// In en, this message translates to:
  /// **'Invite accepted'**
  String get teamRequestAccepted;

  /// No description provided for @teamRequestDenied.
  ///
  /// In en, this message translates to:
  /// **'Invite denied'**
  String get teamRequestDenied;

  /// No description provided for @teamRequestCancelled.
  ///
  /// In en, this message translates to:
  /// **'Invite cancelled'**
  String get teamRequestCancelled;

  /// No description provided for @teamActionError.
  ///
  /// In en, this message translates to:
  /// **'Action failed. Please try again.'**
  String get teamActionError;

  /// No description provided for @exerciseOwnerName.
  ///
  /// In en, this message translates to:
  /// **'Owner Name'**
  String get exerciseOwnerName;

  /// No description provided for @exerciseCategory.
  ///
  /// In en, this message translates to:
  /// **'Category'**
  String get exerciseCategory;

  /// No description provided for @categoryForce.
  ///
  /// In en, this message translates to:
  /// **'Force'**
  String get categoryForce;

  /// No description provided for @categoryCardio.
  ///
  /// In en, this message translates to:
  /// **'Cardio'**
  String get categoryCardio;

  /// No description provided for @workoutDuration.
  ///
  /// In en, this message translates to:
  /// **'Duration'**
  String get workoutDuration;

  /// No description provided for @sessionsTitle.
  ///
  /// In en, this message translates to:
  /// **'Workout Sessions'**
  String get sessionsTitle;

  /// No description provided for @sessionsCount.
  ///
  /// In en, this message translates to:
  /// **'Sessions'**
  String get sessionsCount;

  /// No description provided for @sessionsFilterLastWeek.
  ///
  /// In en, this message translates to:
  /// **'Last Week'**
  String get sessionsFilterLastWeek;

  /// No description provided for @sessionsFilterLastMonth.
  ///
  /// In en, this message translates to:
  /// **'Last Month'**
  String get sessionsFilterLastMonth;

  /// No description provided for @sessionsFilterLast3Months.
  ///
  /// In en, this message translates to:
  /// **'Last 3 Months'**
  String get sessionsFilterLast3Months;

  /// No description provided for @sessionsFilterCustom.
  ///
  /// In en, this message translates to:
  /// **'Custom Range'**
  String get sessionsFilterCustom;

  /// No description provided for @sessionsFilterStartDate.
  ///
  /// In en, this message translates to:
  /// **'Start Date'**
  String get sessionsFilterStartDate;

  /// No description provided for @sessionsFilterEndDate.
  ///
  /// In en, this message translates to:
  /// **'End Date'**
  String get sessionsFilterEndDate;

  /// No description provided for @sessionsNoData.
  ///
  /// In en, this message translates to:
  /// **'No sessions found for the selected period.'**
  String get sessionsNoData;

  /// No description provided for @sessionsViewProgress.
  ///
  /// In en, this message translates to:
  /// **'View My Progress'**
  String get sessionsViewProgress;

  /// No description provided for @sessionsChartVolumeTitle.
  ///
  /// In en, this message translates to:
  /// **'Volume per Session (kg)'**
  String get sessionsChartVolumeTitle;

  /// No description provided for @dayMonday.
  ///
  /// In en, this message translates to:
  /// **'Monday'**
  String get dayMonday;

  /// No description provided for @dayTuesday.
  ///
  /// In en, this message translates to:
  /// **'Tuesday'**
  String get dayTuesday;

  /// No description provided for @dayWednesday.
  ///
  /// In en, this message translates to:
  /// **'Wednesday'**
  String get dayWednesday;

  /// No description provided for @dayThursday.
  ///
  /// In en, this message translates to:
  /// **'Thursday'**
  String get dayThursday;

  /// No description provided for @dayFriday.
  ///
  /// In en, this message translates to:
  /// **'Friday'**
  String get dayFriday;

  /// No description provided for @daySaturday.
  ///
  /// In en, this message translates to:
  /// **'Saturday'**
  String get daySaturday;

  /// No description provided for @daySunday.
  ///
  /// In en, this message translates to:
  /// **'Sunday'**
  String get daySunday;

  /// No description provided for @feedTitle.
  ///
  /// In en, this message translates to:
  /// **'Feed'**
  String get feedTitle;

  /// No description provided for @feedCreatePost.
  ///
  /// In en, this message translates to:
  /// **'Create Post'**
  String get feedCreatePost;

  /// No description provided for @feedWriteSomething.
  ///
  /// In en, this message translates to:
  /// **'What\'s on your mind?'**
  String get feedWriteSomething;

  /// No description provided for @feedPostButton.
  ///
  /// In en, this message translates to:
  /// **'Post'**
  String get feedPostButton;

  /// No description provided for @feedLiveVideo.
  ///
  /// In en, this message translates to:
  /// **'Live Video'**
  String get feedLiveVideo;

  /// No description provided for @feedPhotoVideo.
  ///
  /// In en, this message translates to:
  /// **'Photo/Video'**
  String get feedPhotoVideo;

  /// No description provided for @feedFeeling.
  ///
  /// In en, this message translates to:
  /// **'Feeling'**
  String get feedFeeling;

  /// No description provided for @feedLike.
  ///
  /// In en, this message translates to:
  /// **'Like'**
  String get feedLike;

  /// No description provided for @feedLove.
  ///
  /// In en, this message translates to:
  /// **'Love'**
  String get feedLove;

  /// No description provided for @feedHaha.
  ///
  /// In en, this message translates to:
  /// **'Haha'**
  String get feedHaha;

  /// No description provided for @feedWow.
  ///
  /// In en, this message translates to:
  /// **'Wow'**
  String get feedWow;

  /// No description provided for @feedSad.
  ///
  /// In en, this message translates to:
  /// **'Sad'**
  String get feedSad;

  /// No description provided for @feedAngry.
  ///
  /// In en, this message translates to:
  /// **'Angry'**
  String get feedAngry;

  /// No description provided for @feedComment.
  ///
  /// In en, this message translates to:
  /// **'Comment'**
  String get feedComment;

  /// No description provided for @feedShare.
  ///
  /// In en, this message translates to:
  /// **'Share'**
  String get feedShare;

  /// No description provided for @feedAddComment.
  ///
  /// In en, this message translates to:
  /// **'Write a comment...'**
  String get feedAddComment;

  /// No description provided for @feedNoComments.
  ///
  /// In en, this message translates to:
  /// **'No comments yet'**
  String get feedNoComments;

  /// No description provided for @feedNoPostsYet.
  ///
  /// In en, this message translates to:
  /// **'No posts yet'**
  String get feedNoPostsYet;

  /// No description provided for @feedStartSharing.
  ///
  /// In en, this message translates to:
  /// **'Be the first to share something with your gym community!'**
  String get feedStartSharing;

  /// No description provided for @feedPostSuccess.
  ///
  /// In en, this message translates to:
  /// **'Post created successfully!'**
  String get feedPostSuccess;

  /// No description provided for @feedPostError.
  ///
  /// In en, this message translates to:
  /// **'Failed to create post. Please try again.'**
  String get feedPostError;

  /// No description provided for @feedLoadError.
  ///
  /// In en, this message translates to:
  /// **'Failed to load feed. Please try again.'**
  String get feedLoadError;

  /// No description provided for @feedComments.
  ///
  /// In en, this message translates to:
  /// **'comments'**
  String get feedComments;

  /// No description provided for @feedReactions.
  ///
  /// In en, this message translates to:
  /// **'reactions'**
  String get feedReactions;

  /// No description provided for @feedAddPhoto.
  ///
  /// In en, this message translates to:
  /// **'Photo'**
  String get feedAddPhoto;

  /// No description provided for @feedAddVideo.
  ///
  /// In en, this message translates to:
  /// **'Video'**
  String get feedAddVideo;

  /// No description provided for @feedUploading.
  ///
  /// In en, this message translates to:
  /// **'Uploading media...'**
  String get feedUploading;

  /// No description provided for @feedShareWorkout.
  ///
  /// In en, this message translates to:
  /// **'Share Workout'**
  String get feedShareWorkout;

  /// No description provided for @feedShareWorkoutSummary.
  ///
  /// In en, this message translates to:
  /// **'Just completed a workout!'**
  String get feedShareWorkoutSummary;

  /// No description provided for @sessionDetailExercises.
  ///
  /// In en, this message translates to:
  /// **'Exercises'**
  String get sessionDetailExercises;

  /// No description provided for @sessionDetailStartAgain.
  ///
  /// In en, this message translates to:
  /// **'Start Again'**
  String get sessionDetailStartAgain;

  /// No description provided for @sessionDetailSpeed.
  ///
  /// In en, this message translates to:
  /// **'Speed'**
  String get sessionDetailSpeed;

  /// No description provided for @menuExercises.
  ///
  /// In en, this message translates to:
  /// **'Exercises'**
  String get menuExercises;

  /// No description provided for @labelFilterExercises.
  ///
  /// In en, this message translates to:
  /// **'Filter Exercises'**
  String get labelFilterExercises;

  /// No description provided for @labelOwnerName.
  ///
  /// In en, this message translates to:
  /// **'Owner Name'**
  String get labelOwnerName;

  /// No description provided for @labelCategory.
  ///
  /// In en, this message translates to:
  /// **'Category'**
  String get labelCategory;

  /// No description provided for @labelVisibility.
  ///
  /// In en, this message translates to:
  /// **'Visibility'**
  String get labelVisibility;

  /// No description provided for @labelSort.
  ///
  /// In en, this message translates to:
  /// **'Sort By'**
  String get labelSort;

  /// No description provided for @buttonSaveAsWorkout.
  ///
  /// In en, this message translates to:
  /// **'Save as Workout'**
  String get buttonSaveAsWorkout;

  /// No description provided for @buttonStartWorkout.
  ///
  /// In en, this message translates to:
  /// **'Start Workout'**
  String get buttonStartWorkout;

  /// No description provided for @buttonApplyFilters.
  ///
  /// In en, this message translates to:
  /// **'Apply Filters'**
  String get buttonApplyFilters;

  /// No description provided for @messageExercisesSelected.
  ///
  /// In en, this message translates to:
  /// **'exercises selected'**
  String get messageExercisesSelected;

  /// No description provided for @messageNoExercisesSelected.
  ///
  /// In en, this message translates to:
  /// **'No exercises selected'**
  String get messageNoExercisesSelected;

  /// No description provided for @messageNoExercisesFound.
  ///
  /// In en, this message translates to:
  /// **'No exercises found'**
  String get messageNoExercisesFound;

  /// No description provided for @messageExercisesAvailable.
  ///
  /// In en, this message translates to:
  /// **'Exercises'**
  String get messageExercisesAvailable;

  /// No description provided for @messageSelectAtLeastOne.
  ///
  /// In en, this message translates to:
  /// **'Please select at least one exercise'**
  String get messageSelectAtLeastOne;

  /// No description provided for @messageStartWorkoutFeatureComing.
  ///
  /// In en, this message translates to:
  /// **'Start workout feature coming soon'**
  String get messageStartWorkoutFeatureComing;

  /// No description provided for @tooltipSwipeToView.
  ///
  /// In en, this message translates to:
  /// **'Go back and swipe an exercise right to add it'**
  String get tooltipSwipeToView;

  /// No description provided for @tooltipBackToExercises.
  ///
  /// In en, this message translates to:
  /// **'Back to exercises'**
  String get tooltipBackToExercises;

  /// No description provided for @sortCreatedAtDesc.
  ///
  /// In en, this message translates to:
  /// **'Newest First'**
  String get sortCreatedAtDesc;

  /// No description provided for @sortCreatedAtAsc.
  ///
  /// In en, this message translates to:
  /// **'Oldest First'**
  String get sortCreatedAtAsc;

  /// No description provided for @sortOwnerNameAsc.
  ///
  /// In en, this message translates to:
  /// **'Owner Name (A-Z)'**
  String get sortOwnerNameAsc;

  /// No description provided for @sortNameAsc.
  ///
  /// In en, this message translates to:
  /// **'Exercise Name (A-Z)'**
  String get sortNameAsc;

  /// No description provided for @labelSearchOwners.
  ///
  /// In en, this message translates to:
  /// **'Filter by Owner'**
  String get labelSearchOwners;

  /// No description provided for @labelSelectedOwners.
  ///
  /// In en, this message translates to:
  /// **'owners selected'**
  String get labelSelectedOwners;

  /// No description provided for @messageSearchHint.
  ///
  /// In en, this message translates to:
  /// **'Search people by name...'**
  String get messageSearchHint;

  /// No description provided for @messageNoPersonsFound.
  ///
  /// In en, this message translates to:
  /// **'No people found for that search.'**
  String get messageNoPersonsFound;

  /// No description provided for @messageUserDataNotLoaded.
  ///
  /// In en, this message translates to:
  /// **'User data not loaded. Please try again.'**
  String get messageUserDataNotLoaded;

  /// No description provided for @messageWorkoutSavedSuccessfully.
  ///
  /// In en, this message translates to:
  /// **'Workout saved successfully'**
  String get messageWorkoutSavedSuccessfully;

  /// No description provided for @tooltipAddExercise.
  ///
  /// In en, this message translates to:
  /// **'Add Exercise'**
  String get tooltipAddExercise;

  /// No description provided for @tooltipChangeLanguage.
  ///
  /// In en, this message translates to:
  /// **'Change language'**
  String get tooltipChangeLanguage;

  /// No description provided for @tooltipRefresh.
  ///
  /// In en, this message translates to:
  /// **'Refresh'**
  String get tooltipRefresh;

  /// No description provided for @tooltipPreviousExercise.
  ///
  /// In en, this message translates to:
  /// **'Previous exercise'**
  String get tooltipPreviousExercise;

  /// No description provided for @labelSettings.
  ///
  /// In en, this message translates to:
  /// **'Settings'**
  String get labelSettings;

  /// No description provided for @messageAddToSelection.
  ///
  /// In en, this message translates to:
  /// **'Add to selection'**
  String get messageAddToSelection;

  /// No description provided for @messageRemoveFromSelection.
  ///
  /// In en, this message translates to:
  /// **'Remove'**
  String get messageRemoveFromSelection;

  /// No description provided for @labelTakePhoto.
  ///
  /// In en, this message translates to:
  /// **'Take Photo'**
  String get labelTakePhoto;

  /// No description provided for @labelRecordVideo.
  ///
  /// In en, this message translates to:
  /// **'Record Video'**
  String get labelRecordVideo;

  /// No description provided for @labelVideo.
  ///
  /// In en, this message translates to:
  /// **'VIDEO'**
  String get labelVideo;

  /// No description provided for @buttonStartSession.
  ///
  /// In en, this message translates to:
  /// **'Start Session'**
  String get buttonStartSession;

  /// No description provided for @evolutionTitle.
  ///
  /// In en, this message translates to:
  /// **'Evolution'**
  String get evolutionTitle;

  /// No description provided for @evolutionFilterLastWeek.
  ///
  /// In en, this message translates to:
  /// **'Last Week'**
  String get evolutionFilterLastWeek;

  /// No description provided for @evolutionFilterLastMonth.
  ///
  /// In en, this message translates to:
  /// **'Last Month'**
  String get evolutionFilterLastMonth;

  /// No description provided for @evolutionFilterLast6Months.
  ///
  /// In en, this message translates to:
  /// **'Last 6 Months'**
  String get evolutionFilterLast6Months;

  /// No description provided for @evolutionFilterCustom.
  ///
  /// In en, this message translates to:
  /// **'Custom Range'**
  String get evolutionFilterCustom;

  /// No description provided for @evolutionCheckinsTitle.
  ///
  /// In en, this message translates to:
  /// **'Check-ins'**
  String get evolutionCheckinsTitle;

  /// No description provided for @evolutionLatestMetrics.
  ///
  /// In en, this message translates to:
  /// **'Latest Metrics'**
  String get evolutionLatestMetrics;

  /// No description provided for @evolutionCompositionWeight.
  ///
  /// In en, this message translates to:
  /// **'Weight'**
  String get evolutionCompositionWeight;

  /// No description provided for @evolutionCompositionBodyFat.
  ///
  /// In en, this message translates to:
  /// **'Body Fat'**
  String get evolutionCompositionBodyFat;

  /// No description provided for @evolutionCompositionMuscleMass.
  ///
  /// In en, this message translates to:
  /// **'Muscle Mass'**
  String get evolutionCompositionMuscleMass;

  /// No description provided for @evolutionCompositionVisceralFat.
  ///
  /// In en, this message translates to:
  /// **'Visceral Fat'**
  String get evolutionCompositionVisceralFat;

  /// No description provided for @evolutionCircumferenceChest.
  ///
  /// In en, this message translates to:
  /// **'Chest'**
  String get evolutionCircumferenceChest;

  /// No description provided for @evolutionCircumferenceWaist.
  ///
  /// In en, this message translates to:
  /// **'Waist'**
  String get evolutionCircumferenceWaist;

  /// No description provided for @evolutionCircumferenceHip.
  ///
  /// In en, this message translates to:
  /// **'Hip'**
  String get evolutionCircumferenceHip;

  /// No description provided for @evolutionNoData.
  ///
  /// In en, this message translates to:
  /// **'No evolution data for the selected period.'**
  String get evolutionNoData;

  /// No description provided for @evolutionNoNote.
  ///
  /// In en, this message translates to:
  /// **'No note'**
  String get evolutionNoNote;

  /// No description provided for @evolutionAddCheckin.
  ///
  /// In en, this message translates to:
  /// **'Add Check-in'**
  String get evolutionAddCheckin;

  /// No description provided for @evolutionFormNote.
  ///
  /// In en, this message translates to:
  /// **'Note'**
  String get evolutionFormNote;

  /// No description provided for @evolutionFormNotePlaceholder.
  ///
  /// In en, this message translates to:
  /// **'e.g. Weekly update'**
  String get evolutionFormNotePlaceholder;

  /// No description provided for @evolutionFormVisibility.
  ///
  /// In en, this message translates to:
  /// **'Visibility'**
  String get evolutionFormVisibility;

  /// No description provided for @evolutionSectionComposition.
  ///
  /// In en, this message translates to:
  /// **'Body Composition'**
  String get evolutionSectionComposition;

  /// No description provided for @evolutionSectionCircumferences.
  ///
  /// In en, this message translates to:
  /// **'Circumferences'**
  String get evolutionSectionCircumferences;

  /// No description provided for @evolutionCircumferenceNeck.
  ///
  /// In en, this message translates to:
  /// **'Neck'**
  String get evolutionCircumferenceNeck;

  /// No description provided for @evolutionCircumferenceAbdomen.
  ///
  /// In en, this message translates to:
  /// **'Abdomen'**
  String get evolutionCircumferenceAbdomen;

  /// No description provided for @evolutionCircumferenceBicepsRight.
  ///
  /// In en, this message translates to:
  /// **'Biceps Right'**
  String get evolutionCircumferenceBicepsRight;

  /// No description provided for @evolutionCircumferenceBicepsLeft.
  ///
  /// In en, this message translates to:
  /// **'Biceps Left'**
  String get evolutionCircumferenceBicepsLeft;

  /// No description provided for @evolutionCircumferenceThighRight.
  ///
  /// In en, this message translates to:
  /// **'Thigh Right'**
  String get evolutionCircumferenceThighRight;

  /// No description provided for @evolutionCircumferenceThighLeft.
  ///
  /// In en, this message translates to:
  /// **'Thigh Left'**
  String get evolutionCircumferenceThighLeft;

  /// No description provided for @evolutionAtLeastOneMetric.
  ///
  /// In en, this message translates to:
  /// **'Add at least one metric before saving.'**
  String get evolutionAtLeastOneMetric;

  /// No description provided for @evolutionCreatedSuccessfully.
  ///
  /// In en, this message translates to:
  /// **'Evolution check-in created successfully'**
  String get evolutionCreatedSuccessfully;

  /// No description provided for @evolutionVitruvianTitle.
  ///
  /// In en, this message translates to:
  /// **'Body Progress (Vitruvian)'**
  String get evolutionVitruvianTitle;

  /// No description provided for @evolutionVitruvianLowChange.
  ///
  /// In en, this message translates to:
  /// **'Low change'**
  String get evolutionVitruvianLowChange;

  /// No description provided for @evolutionVitruvianHighChange.
  ///
  /// In en, this message translates to:
  /// **'High change'**
  String get evolutionVitruvianHighChange;

  /// No description provided for @evolutionVitruvianInterpolationNote.
  ///
  /// In en, this message translates to:
  /// **'Some regions include interpolated values where check-ins had missing fields.'**
  String get evolutionVitruvianInterpolationNote;

  /// No description provided for @evolutionVitruvianBaselineProfileCheckin.
  ///
  /// In en, this message translates to:
  /// **'baseline: profile + first check-in fallback'**
  String get evolutionVitruvianBaselineProfileCheckin;

  /// No description provided for @evolutionVitruvianBaselineCheckinOnly.
  ///
  /// In en, this message translates to:
  /// **'baseline: first check-in only'**
  String get evolutionVitruvianBaselineCheckinOnly;

  /// No description provided for @evolutionVitruvianSetGenderHint.
  ///
  /// In en, this message translates to:
  /// **'Set your gender in your profile for a more accurate model.'**
  String get evolutionVitruvianSetGenderHint;

  /// No description provided for @evolutionVitruvianShoulders.
  ///
  /// In en, this message translates to:
  /// **'Shoulders'**
  String get evolutionVitruvianShoulders;

  /// No description provided for @evolutionVitruvianArms.
  ///
  /// In en, this message translates to:
  /// **'Arms'**
  String get evolutionVitruvianArms;

  /// No description provided for @evolutionVitruvianAbdomen.
  ///
  /// In en, this message translates to:
  /// **'Abdomen'**
  String get evolutionVitruvianAbdomen;

  /// No description provided for @evolutionVitruvianHips.
  ///
  /// In en, this message translates to:
  /// **'Hips'**
  String get evolutionVitruvianHips;

  /// No description provided for @evolutionVitruvianThighs.
  ///
  /// In en, this message translates to:
  /// **'Thighs'**
  String get evolutionVitruvianThighs;

  /// No description provided for @evolutionVitruvianViewFront.
  ///
  /// In en, this message translates to:
  /// **'Front'**
  String get evolutionVitruvianViewFront;

  /// No description provided for @evolutionVitruvianViewBack.
  ///
  /// In en, this message translates to:
  /// **'Back'**
  String get evolutionVitruvianViewBack;

  /// No description provided for @evolutionVitruvianBack.
  ///
  /// In en, this message translates to:
  /// **'Back'**
  String get evolutionVitruvianBack;

  /// No description provided for @evolutionVitruvianUpperBack.
  ///
  /// In en, this message translates to:
  /// **'Upper back'**
  String get evolutionVitruvianUpperBack;

  /// No description provided for @evolutionVitruvianTriceps.
  ///
  /// In en, this message translates to:
  /// **'Triceps'**
  String get evolutionVitruvianTriceps;

  /// No description provided for @evolutionVitruvianGlutes.
  ///
  /// In en, this message translates to:
  /// **'Glutes'**
  String get evolutionVitruvianGlutes;

  /// No description provided for @evolutionVitruvianCalves.
  ///
  /// In en, this message translates to:
  /// **'Calves'**
  String get evolutionVitruvianCalves;

  /// No description provided for @buttonRetry.
  ///
  /// In en, this message translates to:
  /// **'Retry'**
  String get buttonRetry;

  /// No description provided for @exerciseCreateSuccess.
  ///
  /// In en, this message translates to:
  /// **'Exercise \"{name}\" created successfully!'**
  String exerciseCreateSuccess(String name);

  /// No description provided for @exerciseUserDataUnavailable.
  ///
  /// In en, this message translates to:
  /// **'User data not available'**
  String get exerciseUserDataUnavailable;

  /// No description provided for @exerciseCreateFailed.
  ///
  /// In en, this message translates to:
  /// **'Failed to create exercise. Please try again.'**
  String get exerciseCreateFailed;

  /// No description provided for @exerciseDurationMin.
  ///
  /// In en, this message translates to:
  /// **'Duration (min)'**
  String get exerciseDurationMin;

  /// No description provided for @addProfileTitle.
  ///
  /// In en, this message translates to:
  /// **'Add a new profile'**
  String get addProfileTitle;

  /// No description provided for @addProfileSubtitle.
  ///
  /// In en, this message translates to:
  /// **'Choose the type of business profile you want to create'**
  String get addProfileSubtitle;

  /// No description provided for @addProfilePersonalTrainerTitle.
  ///
  /// In en, this message translates to:
  /// **'Personal Trainer'**
  String get addProfilePersonalTrainerTitle;

  /// No description provided for @addProfilePersonalTrainerDescription.
  ///
  /// In en, this message translates to:
  /// **'Offer personal training services and manage your own clients'**
  String get addProfilePersonalTrainerDescription;

  /// No description provided for @addProfileGymTitle.
  ///
  /// In en, this message translates to:
  /// **'Gym'**
  String get addProfileGymTitle;

  /// No description provided for @addProfileGymDescription.
  ///
  /// In en, this message translates to:
  /// **'Manage a gym or fitness facility and its members'**
  String get addProfileGymDescription;

  /// No description provided for @businessProfileFormTitlePersonalTrainer.
  ///
  /// In en, this message translates to:
  /// **'Create your Personal Trainer profile'**
  String get businessProfileFormTitlePersonalTrainer;

  /// No description provided for @businessProfileFormTitleGym.
  ///
  /// In en, this message translates to:
  /// **'Create your Gym profile'**
  String get businessProfileFormTitleGym;

  /// No description provided for @businessProfileFormBusinessName.
  ///
  /// In en, this message translates to:
  /// **'Business name'**
  String get businessProfileFormBusinessName;

  /// No description provided for @businessProfileFormBusinessNameRequired.
  ///
  /// In en, this message translates to:
  /// **'Please enter a business name'**
  String get businessProfileFormBusinessNameRequired;

  /// No description provided for @businessProfileFormSocialName.
  ///
  /// In en, this message translates to:
  /// **'Social name'**
  String get businessProfileFormSocialName;

  /// No description provided for @businessProfileFormSocialNameRequired.
  ///
  /// In en, this message translates to:
  /// **'Please enter a social name'**
  String get businessProfileFormSocialNameRequired;

  /// No description provided for @businessProfileFormTaxId.
  ///
  /// In en, this message translates to:
  /// **'Tax ID'**
  String get businessProfileFormTaxId;

  /// No description provided for @businessProfileFormTaxIdRequired.
  ///
  /// In en, this message translates to:
  /// **'Please enter a tax ID'**
  String get businessProfileFormTaxIdRequired;

  /// No description provided for @businessProfileFormSubmit.
  ///
  /// In en, this message translates to:
  /// **'Create profile'**
  String get businessProfileFormSubmit;

  /// No description provided for @businessProfileFormError.
  ///
  /// In en, this message translates to:
  /// **'Failed to create business profile. Please try again.'**
  String get businessProfileFormError;

  /// No description provided for @businessProfilePageEdit.
  ///
  /// In en, this message translates to:
  /// **'Edit'**
  String get businessProfilePageEdit;

  /// No description provided for @businessProfilePageSave.
  ///
  /// In en, this message translates to:
  /// **'Save'**
  String get businessProfilePageSave;

  /// No description provided for @businessProfilePageAddresses.
  ///
  /// In en, this message translates to:
  /// **'Addresses'**
  String get businessProfilePageAddresses;

  /// No description provided for @businessProfileSwitchToPersonal.
  ///
  /// In en, this message translates to:
  /// **'Personal account'**
  String get businessProfileSwitchToPersonal;
}

class _AppLocalizationsDelegate
    extends LocalizationsDelegate<AppLocalizations> {
  const _AppLocalizationsDelegate();

  @override
  Future<AppLocalizations> load(Locale locale) {
    return SynchronousFuture<AppLocalizations>(lookupAppLocalizations(locale));
  }

  @override
  bool isSupported(Locale locale) =>
      <String>['en', 'es', 'fr', 'nl', 'pt'].contains(locale.languageCode);

  @override
  bool shouldReload(_AppLocalizationsDelegate old) => false;
}

AppLocalizations lookupAppLocalizations(Locale locale) {
  // Lookup logic when language+country codes are specified.
  switch (locale.languageCode) {
    case 'pt':
      {
        switch (locale.countryCode) {
          case 'BR':
            return AppLocalizationsPtBr();
        }
        break;
      }
  }

  // Lookup logic when only language code is specified.
  switch (locale.languageCode) {
    case 'en':
      return AppLocalizationsEn();
    case 'es':
      return AppLocalizationsEs();
    case 'fr':
      return AppLocalizationsFr();
    case 'nl':
      return AppLocalizationsNl();
    case 'pt':
      return AppLocalizationsPt();
  }

  throw FlutterError(
    'AppLocalizations.delegate failed to load unsupported locale "$locale". This is likely '
    'an issue with the localizations generation tool. Please file an issue '
    'on GitHub with a reproducible sample app and the gen-l10n configuration '
    'that was used.',
  );
}
