use crate::http::json::access_token_json::{AccessTokenJson, PendingAccountDeletionJson};
use crate::http::json::business_profile_address_json::BusinessProfileAddressJson;
use crate::http::json::business_profile_json::BusinessProfileJson;
use crate::http::json::country_json::CountryJson;
use crate::http::json::address_candidate_json::AddressCandidateJson;
use crate::http::json::exercise_json::ExerciseJson;
use crate::http::json::person_address_json::PersonAddressJson;
use crate::http::json::person_info_json::PersonInfoJson;
use crate::http::json::person_json::PersonJson;
use crate::http::json::team_member_json::TeamMemberJson;
use crate::http::json::user_json::UserJson;
use crate::http::json::workout_json::WorkoutJson;
use business::domain::access_token::AccessToken;
use business::domain::business_profile::BusinessProfile;
use business::domain::business_profile_address::BusinessProfileAddress;
use business::domain::country::Country;
use business::domain::address_candidate::AddressCandidate;
use business::domain::enums::{Difficulty, Position, ProfileType};
use business::domain::exercise::{Category, Exercise};
use business::domain::person::Person;
use business::domain::person_address::PersonAddress;
use business::domain::person_info::PersonInfo;
use business::domain::settings::Settings;
use business::domain::team_member::{TeamMember, TeamMemberStatus};
use business::domain::user::User;
use business::domain::workout::{Visibility, Workout};
use crate::http::json::settings_json::SettingsJson;

pub trait Mapper<T, U> {
    fn json(t: T) -> U;

    fn domain(u: U) -> T;

    fn json_opt(t: Option<T>) -> Option<U> {
        t.map(Self::json)
    }
    
    fn domain_vec(u: Vec<U>) -> Vec<T> {
        u.into_iter().map(Self::domain).collect()
    }

    fn json_vec(u: Vec<T>) -> Vec<U> {
        u.into_iter().map(Self::json).collect()
    }


}

pub struct AccessTokenMapper {}

impl Mapper<AccessToken, AccessTokenJson> for AccessTokenMapper {
    fn json(access_token: AccessToken) -> AccessTokenJson {
        AccessTokenJson {
            access_token: access_token.access_token,
            token_type: access_token.token_type,
            expire_in: access_token.expire_in,
            refresh_token: access_token.refresh_token,
            username: access_token.username,
            uuid: access_token.uuid,
            name: access_token.name,
            person_id: access_token.person_id,
            person_uuid: access_token.person_uuid,
            person_object_key: access_token.person_object_key,
            active_business_profile_id: access_token.active_business_profile_id,
            active_business_profile_uuid: access_token.active_business_profile_uuid,
            pending_account_deletion: access_token.pending_account_deletion.map(|p| {
                PendingAccountDeletionJson {
                    requested_at: p.requested_at,
                    scheduled_at: p.scheduled_at,
                }
            }),
        }
    }

    fn domain(u: AccessTokenJson) -> AccessToken {
        AccessToken {
            access_token: u.access_token,
            token_type: u.token_type,
            expire_in: u.expire_in,
            refresh_token: u.refresh_token,
            username: u.username,
            uuid: u.uuid,
            name: u.name,
            person_id: u.person_id,
            person_uuid: u.person_uuid,
            person_object_key: u.person_object_key,
            active_business_profile_id: u.active_business_profile_id,
            active_business_profile_uuid: u.active_business_profile_uuid,
            pending_account_deletion: None,
        }
    }
}

pub struct PersonMapper {}

impl Mapper<Person, PersonJson> for PersonMapper {
    fn json(person: Person) -> PersonJson {
        PersonJson {
            id: person.id,
            uuid: person.uuid,
            firstname: Some(person.firstname),
            surname: Some(person.surname),
            date_of_birth: Some(person.date_of_birth),
            gender: Some(person.gender),
            avatar: person.avatar,
            object_key: person.object_key,
            cover: person.cover,
            user: person.user.map(UserMapper::json),
            person_info: person.person_info.map(PersonInfoMapper::json),
            addresses: PersonAddressMapper::json_vec(person.addresses),
            created_at: person.created_at,
            updated_at: person.updated_at,
            business_profiles: BusinessProfileMapper::json_vec(person.business_profiles),
        }
    }
    fn domain(u: PersonJson) -> Person {
        Person {
            id: u.id,
            uuid: u.uuid,
            firstname: u.firstname.unwrap_or_default(),
            surname: u.surname.unwrap_or_default(),
            date_of_birth: u.date_of_birth.unwrap_or_default(),
            gender: u.gender.unwrap_or_default(),
            avatar: None,
            object_key: None,
            cover: None,
            user: u.user.map(UserMapper::domain),
            person_info: u.person_info.map(PersonInfoMapper::domain),
            addresses: PersonAddressMapper::domain_vec(u.addresses),
            business_profiles: BusinessProfileMapper::domain_vec(u.business_profiles),
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}

pub struct UserMapper {}

impl Mapper<User, UserJson> for UserMapper {
    fn json(user: User) -> UserJson {
        UserJson {
            id: user.id,
            uuid: user.uuid,
            name: user.name,
            email: user.email,
            password: user.password,
            enabled: user.enabled,
            first_login: user.first_login,
            person_id: Some(user.person_id),
            person_uuid: Some(user.person_uuid),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }

    fn domain(u: UserJson) -> User {
        User {
            id: u.id,
            uuid: u.uuid,
            email: u.email,
            name: u.name,
            password: u.password,
            enabled: u.enabled,
            first_login: u.first_login,
            person_id: u.person_id.unwrap(),
            person_uuid: u.person_uuid.unwrap(),
            created_at: u.created_at,
            updated_at: u.updated_at,
            failed_login_attempts: 0,
            locked_until: None,
            token_valid_after: None,
            deletion_requested_at: None,
            deletion_scheduled_at: None,
        }
    }
}

pub struct PersonInfoMapper {}

impl Mapper<PersonInfo, PersonInfoJson> for PersonInfoMapper {
    fn json(person_info: PersonInfo) -> PersonInfoJson {
        PersonInfoJson {
            id: person_info.id,
            person_id: person_info.person_id,
            biography: person_info.biography,
            relationship: person_info.relationship,
            job: person_info.job,
            home_town: person_info.home_town,
            current_city: person_info.current_city,
            weight: person_info.weight,
            height: person_info.height,
            uuid: person_info.uuid,
            created_at: person_info.created_at,
            updated_at: person_info.updated_at,
        }
    }

    fn domain(u: PersonInfoJson) -> PersonInfo {
        PersonInfo {
            id: u.id,
            person_id: u.person_id,
            biography: u.biography,
            relationship: u.relationship,
            job: u.job,
            home_town: u.home_town,
            current_city: u.current_city,
            weight: u.weight,
            height: u.height,
            uuid: u.uuid,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}

pub struct PersonAddressMapper {}

impl Mapper<PersonAddress, PersonAddressJson> for PersonAddressMapper {
    fn json(address: PersonAddress) -> PersonAddressJson {
        PersonAddressJson {
            id: address.id,
            uuid: address.uuid,
            person_id: address.person_id,
            address_line1: address.address_line1,
            address_line2: address.address_line2,
            administrative_area: address.administrative_area,
            locality: address.locality,
            postal_code: address.postal_code,
            country_code: address.country_code,
            current: address.current,
            latitude: None,
            longitude: None,
            created_at: address.created_at,
            updated_at: address.updated_at,
        }
    }

    fn domain(u: PersonAddressJson) -> PersonAddress {
        PersonAddress {
            id: u.id,
            uuid: u.uuid,
            person_id: u.person_id,
            address_line1: u.address_line1,
            address_line2: u.address_line2,
            administrative_area: u.administrative_area,
            locality: u.locality,
            postal_code: u.postal_code,
            country_code: u.country_code,
            current: u.current,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}

pub struct WorkoutMapper {}

impl Mapper<Workout, WorkoutJson> for WorkoutMapper {
    fn json(workout: Workout) -> WorkoutJson {
        WorkoutJson {
            id: workout.id,
            uuid: workout.uuid,
            name: Some(workout.name),
            description: workout.description,
            difficulty: Some(workout.difficulty.to_string()),
            muscle_group: Some(workout.muscle_group),
            owner_id: workout.owner_id,
            owner_uuid: workout.owner_uuid,
            exercises: ExerciseMapper::json_vec(workout.exercises),
            visibility: workout.visibility.to_string(),
            created_at: workout.created_at,
            updated_at: workout.updated_at,
            target_person_uuid: None,
        }
    }

    fn domain(u: WorkoutJson) -> Workout {
        Workout {
            id: u.id,
            uuid: u.uuid,
            name: u.name.unwrap(),
            description: u.description,
            difficulty: Difficulty::from_string(u.difficulty.unwrap().as_str()),
            muscle_group: u.muscle_group.unwrap(),
            owner_id: u.owner_id,
            owner_uuid: u.owner_uuid,
            exercises: ExerciseMapper::domain_vec(u.exercises),
            visibility: Visibility::from_string(u.visibility.as_str()),
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}

pub struct ExerciseMapper {}

impl Mapper<Exercise, ExerciseJson> for ExerciseMapper {
    fn json(t: Exercise) -> ExerciseJson {
        ExerciseJson {
            id: t.id,
            uuid: t.uuid,
            name: Some(t.name),
            owner_id: t.owner_id,
            owner_name: t.owner_name,
            owner_uuid: t.owner_uuid,
            description: t.description,
            sets: Some(t.sets),
            category: Some(t.category.to_string()),
            reps_or_duration: Some(t.reps_or_duration),
            visibility: t.visibility.to_string(),
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }

    fn domain(u: ExerciseJson) -> Exercise {
        Exercise {
            id: u.id,
            uuid: u.uuid,
            name: u.name.unwrap(),
            owner_id: u.owner_id,
            owner_name: u.owner_name,
            owner_uuid: u.owner_uuid,
            description: u.description,
            sets: u.sets.unwrap(),
            category: Category::from_string(u.category.unwrap().as_str()),
            reps_or_duration: u.reps_or_duration.unwrap(),
            visibility: Visibility::from_string(u.visibility.as_str()),
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}

pub struct BusinessProfileAddressMapper {}

impl Mapper<BusinessProfileAddress, BusinessProfileAddressJson> for BusinessProfileAddressMapper {
    fn json(address: BusinessProfileAddress) -> BusinessProfileAddressJson {
        BusinessProfileAddressJson {
            id: address.id,
            uuid: address.uuid,
            business_profile_id: address.business_profile_id,
            address_line1: address.address_line1,
            address_line2: address.address_line2,
            administrative_area: address.administrative_area,
            postal_code: address.postal_code,
            locality: address.locality,
            country_code: address.country_code,
            longitude: None,
            latitude: None,
        }
    }

    fn domain(u: BusinessProfileAddressJson) -> BusinessProfileAddress {
        BusinessProfileAddress {
            id: u.id,
            uuid: u.uuid,
            business_profile_id: u.business_profile_id,
            address_line1: u.address_line1,
            address_line2: u.address_line2,
            administrative_area: u.administrative_area,
            postal_code: u.postal_code,
            locality: u.locality,
            country_code: u.country_code,
            created_at: None,
            updated_at: None,
        }
    }
}

pub struct BusinessProfileMapper {}

impl Mapper<BusinessProfile, BusinessProfileJson> for BusinessProfileMapper {
    fn json(profile: BusinessProfile) -> BusinessProfileJson {
        BusinessProfileJson {
            id: profile.id,
            uuid: profile.uuid,
            owner_id: profile.owner_id,
            owner_uuid: profile.owner_uuid,
            tax_id: profile.tax_id,
            business_name: profile.business_name,
            business_type: profile.business_type.to_string(),
            social_name: profile.social_name,
            logo: profile.logo,
            object_key: profile.object_key,
            cover_image: profile.cover_image,
            addresses: BusinessProfileAddressMapper::json_vec(profile.addresses),
        }
    }

    fn domain(u: BusinessProfileJson) -> BusinessProfile {
        BusinessProfile {
            id: u.id,
            uuid: u.uuid,
            owner_id: u.owner_id,
            owner_uuid: u.owner_uuid,
            tax_id: u.tax_id,
            business_name: u.business_name,
            business_type: ProfileType::from_string(&u.business_type),
            social_name: u.social_name,
            logo: u.logo,
            object_key: u.object_key,
            cover_image: u.cover_image,
            addresses: BusinessProfileAddressMapper::domain_vec(u.addresses),
            created_at: None,
            updated_at: None,
        }
    }
}

pub struct CountryMapper {}

impl Mapper<Country, CountryJson> for CountryMapper {
    fn json(t: Country) -> CountryJson {
        CountryJson {
            id: t.id,
            ddi: Some(t.ddi),
            name: Some(t.name),
            acronym: Some(t.acronym),
            currency: Some(t.currency),
        }
    }

    fn domain(u: CountryJson) -> Country  {
        Country {
            id: u.id,
            ddi: u.ddi.unwrap(),
            name: u.name.unwrap(),
            acronym: u.acronym.unwrap(),
            currency: u.currency.unwrap(),
        }
    }
}

pub struct AddressCandidateMapper {}

impl Mapper<AddressCandidate, AddressCandidateJson> for AddressCandidateMapper {
    fn json(t: AddressCandidate) -> AddressCandidateJson {
        AddressCandidateJson {
            place_id: t.place_id,
            formatted_address: t.formatted_address,
            address_line1: t.address_line1,
            address_line2: t.address_line2,
            locality: t.locality,
            administrative_area: t.administrative_area,
            administrative_area_code: t.administrative_area_code,
            postal_code: t.postal_code,
            country_code: t.country_code,
            latitude: t.latitude,
            longitude: t.longitude,
        }
    }

    fn domain(u: AddressCandidateJson) -> AddressCandidate {
        AddressCandidate {
            place_id: u.place_id,
            formatted_address: u.formatted_address,
            address_line1: u.address_line1,
            address_line2: u.address_line2,
            locality: u.locality,
            administrative_area: u.administrative_area,
            administrative_area_code: u.administrative_area_code,
            postal_code: u.postal_code,
            country_code: u.country_code,
            latitude: u.latitude,
            longitude: u.longitude,
        }
    }
}

pub struct SettingsMapper {}

impl Mapper<Settings,SettingsJson> for SettingsMapper {
    fn json(t: Settings) -> SettingsJson {
        SettingsJson {
            id: t.id,
            uuid: t.uuid,
            person_id: t.person_id,
            person_uuid: t.person_uuid,
            language: t.language,
            theme: t.theme,
            notifications_enabled: t.notifications_enabled,
            context_menu_position: t.context_menu_position.to_string(),
            home_page: t.home_page,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }

    fn domain(u: SettingsJson) -> Settings {
        Settings {
            id: u.id,
            uuid: u.uuid,
            person_id: u.person_id,
            person_uuid: u.person_uuid,
            language: u.language,
            theme: u.theme,
            notifications_enabled: u.notifications_enabled,
            context_menu_position: Position::from_string(u.context_menu_position.as_str()),
            home_page: u.home_page,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}
pub struct TeamMemberMapper {}

impl Mapper<TeamMember, TeamMemberJson> for TeamMemberMapper {
    fn json(team_member: TeamMember) -> TeamMemberJson {
        TeamMemberJson {
            id: team_member.id,
            uuid: team_member.uuid,
            business_profile_id: team_member.business_profile_id,
            business_profile_uuid: team_member.business_profile_uuid,
            person_id: team_member.person_id,
            person_uuid: team_member.person_uuid,
            status: team_member.status.as_str(),
        }
    }

    fn domain(u: TeamMemberJson) -> TeamMember {
        TeamMember {
            id: u.id,
            uuid: u.uuid,
            business_profile_id: u.business_profile_id,
            business_profile_uuid: u.business_profile_uuid,
            person_id: u.person_id,
            person_uuid: u.person_uuid,
            status: TeamMemberStatus::from_string(&u.status),
            created_at: None,
            updated_at: None,
        }
    }
}
