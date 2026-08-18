use business::domain::{
    business_profile::BusinessProfile as DomainBusinessProfile,
    business_profile_address::BusinessProfileAddress as DomainBusinessProfileAddress,
    country::Country as DomainCountry, exercise::Exercise as DomainExercise,
    friend::Friend as DomainFriend, person::Person as DomainPerson,
    person_address::PersonAddress as DomainPersonAddress,
    person_info::PersonInfo as DomainPersonInfo,
    user::User as DomainUser, workout::Workout as DomainWorkout,
    settings::Settings as DomainSettings,
    team_member::{TeamMember as DomainTeamMember, TeamMemberStatus},
    profile::Profile as DomainProfile
};
use chrono::NaiveDateTime;
use business::domain::enums::{Position, ProfileType};
use crate::proto;

// ---------------------------------------------------------------------------
// Trait
// ---------------------------------------------------------------------------

pub trait Mapper<D, RESPONSE> {
    fn response(t: D) -> RESPONSE;

    fn response_option(t: Option<D>) -> Option<RESPONSE> {
        t.map(Self::response)
    }

    fn response_vec(u: Vec<D>) -> Vec<RESPONSE> {
        u.into_iter().map(Self::response).collect()
    }

    fn domain(u: RESPONSE) -> D;

    fn domain_option(u: Option<RESPONSE>) -> Option<D> {
        u.map(Self::domain)
    }

    fn domain_vec(u: Vec<RESPONSE>) -> Vec<D> {
        u.into_iter().map(Self::domain).collect()
    }

}

// ---------------------------------------------------------------------------
// Mapper structs
// ---------------------------------------------------------------------------

pub struct UserMapper;
pub struct PersonInfoMapper;
pub struct PersonAddressMapper;
pub struct PersonMapper;
pub struct FriendMapper;
pub struct ProvinceMapper;
pub struct CountryMapper;
pub struct ExerciseMapper;
pub struct WorkoutMapper;
pub struct BusinessProfileAddressMapper;
pub struct BusinessProfileMapper;

pub struct SettingsMapper;
pub struct TeamMemberMapper;

// ---------------------------------------------------------------------------
// UserMapper
// ---------------------------------------------------------------------------

impl Mapper<DomainUser, proto::user::User> for UserMapper {
    fn response(t: DomainUser) -> proto::user::User {
        proto::user::User  {
            id: t.id.unwrap_or(0),
            uuid: t.uuid.unwrap_or_default(),
            name: t.name.unwrap_or_default(),
            email: t.email,
            password: t.password,
        enabled: t.enabled,
            first_login: t.first_login,
            person_id: t.person_id,
            person_uuid: t.person_uuid,
            created_at: t.created_at.map(|d| d.to_string()).unwrap_or_default(),
            updated_at: t.updated_at.map(|d| d.to_string()).unwrap_or_default(),
        }
    }

    fn domain(u: proto::user::User) -> DomainUser {
        DomainUser {
            id: if u.id == 0 { None } else { Some(u.id) },
            uuid: if u.uuid.is_empty() { None } else { Some(u.uuid) },
            name: Some(u.name),
            email: u.email,
            password: u.password,
            enabled: u.enabled,
            first_login: u.first_login,
            person_id: u.person_id,
            person_uuid: u.person_uuid,
            created_at: NaiveDateTime::parse_from_str(&u.created_at, "%Y-%m-%d %H:%M:%S%.f").ok(),
            updated_at: NaiveDateTime::parse_from_str(&u.updated_at, "%Y-%m-%d %H:%M:%S%.f").ok(),
            failed_login_attempts: 0,
            locked_until: None,
            token_valid_after: None,
        }
    }
}

// ---------------------------------------------------------------------------
// PersonInfoMapper
// ---------------------------------------------------------------------------

impl Mapper<DomainPersonInfo, proto::person_info::PersonInfo> for PersonInfoMapper {
    fn response(t: DomainPersonInfo) -> proto::person_info::PersonInfo {
        proto::person_info::PersonInfo {
            id: t.id.unwrap_or(0),
            person_id: t.person_id,
            biography: t.biography.unwrap_or_default(),
            relationship: t.relationship.unwrap_or_default(),
            job: t.job.unwrap_or_default(),
            home_town: t.home_town.unwrap_or_default(),
            current_city: t.current_city.unwrap_or_default(),
            weight: t.weight.unwrap_or(0.0),
            height: t.height.unwrap_or(0.0),
            uuid: t.uuid.unwrap_or_default(),
            created_at: t.created_at.map(|d| d.to_string()).unwrap_or_default(),
            updated_at: t.updated_at.map(|d| d.to_string()).unwrap_or_default(),
        }
    }

    fn domain(u: proto::person_info::PersonInfo) -> DomainPersonInfo {
        DomainPersonInfo {
            id: if u.id == 0 { None } else { Some(u.id) },
            uuid: if u.uuid.is_empty() { None } else { Some(u.uuid) },
            person_id: u.person_id,
            biography: Some(u.biography),
            relationship: Some(u.relationship),
            job: Some(u.job),
            home_town: Some(u.home_town),
            current_city: Some(u.current_city),
            weight: Some(u.weight),
            height: Some(u.height),
            created_at: NaiveDateTime::parse_from_str(&u.created_at, "%Y-%m-%d %H:%M:%S%.f").ok(),
            updated_at: NaiveDateTime::parse_from_str(&u.updated_at, "%Y-%m-%d %H:%M:%S%.f").ok(),
        }
    }
}

// ---------------------------------------------------------------------------
// PersonAddressMapper
// ---------------------------------------------------------------------------

impl Mapper<DomainPersonAddress, proto::person_address::PersonAddress> for PersonAddressMapper {
    fn response(t: DomainPersonAddress) -> proto::person_address::PersonAddress {
        proto::person_address::PersonAddress {
            id: t.id.unwrap_or(0),
            person_id: t.person_id,
            address_line_1: t.address_line1,
            address_line_2: t.address_line2.unwrap_or_default(),
            locality: t.locality,
            administrative_area: t.administrative_area,
            postal_code: t.postal_code.unwrap_or_default(),
            country_code: t.country_code,
            latitude: t.latitude.unwrap_or(0.0),
            longitude: t.longitude.unwrap_or(0.0),
            current: t.current,
            uuid: t.uuid.unwrap_or_default(),
            created_at: t.created_at.map(|d| d.to_string()).unwrap_or_default(),
            updated_at: t.updated_at.map(|d| d.to_string()).unwrap_or_default(),
        }
    }

    fn domain(u: proto::person_address::PersonAddress) -> DomainPersonAddress {
        DomainPersonAddress {
            id: if u.id == 0 { None } else { Some(u.id) },
            uuid: if u.uuid.is_empty() { None } else { Some(u.uuid) },
            person_id: u.person_id,
            current: u.current,
            latitude: Some(u.latitude),
            longitude: Some(u.longitude),
            address_line1: u.address_line_1,
            address_line2: if u.address_line_2.is_empty() { None } else { Some(u.address_line_2) },
            locality: u.locality,
            administrative_area: u.administrative_area,
            postal_code: if u.postal_code.is_empty() { None } else { Some(u.postal_code) },
            created_at: NaiveDateTime::parse_from_str(&u.created_at, "%Y-%m-%d %H:%M:%S%.f").ok(),
            updated_at: NaiveDateTime::parse_from_str(&u.updated_at, "%Y-%m-%d %H:%M:%S%.f").ok(),
            country_code: u.country_code,
        }
    }
}

// ---------------------------------------------------------------------------
// PersonMapper
// ---------------------------------------------------------------------------

impl Mapper<DomainPerson, proto::person::Person> for PersonMapper {
    fn response(t: DomainPerson) -> proto::person::Person {
        proto::person::Person {
            id: t.id.unwrap_or(0),
            uuid: t.uuid.unwrap_or_default(),
            firstname: t.firstname,
            surname: t.surname,
            date_of_birth: t.date_of_birth.to_string(),
            gender: t.gender,
            object_key: t.object_key.unwrap_or_default(),
            avatar: t.avatar.unwrap_or_default(),
            cover: t.cover.unwrap_or_default(),
            user: t.user.map(UserMapper::response),
            person_info: t.person_info.map(PersonInfoMapper::response),
            addresses: PersonAddressMapper::response_vec(t.addresses),
            business_profiles: BusinessProfileMapper::response_vec(t.business_profiles),
            created_at: t.created_at.map(|d| d.to_string()).unwrap_or_default(),
            updated_at: t.updated_at.map(|d| d.to_string()).unwrap_or_default(),
        }
    }

    fn domain(u: proto::person::Person) -> DomainPerson {
        DomainPerson {
            id: if u.id == 0 { None } else { Some(u.id) },
            uuid: if u.uuid.is_empty() { None } else { Some(u.uuid) },
            firstname: u.firstname,
            surname: u.surname,
            date_of_birth: chrono::NaiveDate::parse_from_str(&u.date_of_birth, "%Y-%m-%d").unwrap_or_default(),
            gender: u.gender,
            object_key: Some(u.object_key),
            avatar: Some(u.avatar),
            cover: Some(u.cover),
            user: u.user.map(UserMapper::domain),
            person_info: u.person_info.map(PersonInfoMapper::domain),
            addresses: PersonAddressMapper::domain_vec(u.addresses),
            business_profiles: BusinessProfileMapper::domain_vec(u.business_profiles),
            created_at: NaiveDateTime::parse_from_str(&u.created_at, "%Y-%m-%d %H:%M:%S%.f").ok(),
            updated_at: NaiveDateTime::parse_from_str(&u.updated_at, "%Y-%m-%d %H:%M:%S%.f").ok(),
        }
    }
}

// ---------------------------------------------------------------------------
// FriendMapper
// ---------------------------------------------------------------------------

impl Mapper<DomainFriend, proto::friend::Friend> for FriendMapper {
    fn response(t: DomainFriend) -> proto::friend::Friend {
        proto::friend::Friend {
            id: t.id.unwrap_or(0),
            uuid: t.uuid.unwrap(),
            person_id: t.person_id,
            person_uuid: t.person_uuid,
            friend_id: t.friend_id,
            friend_uuid: t.friend_uuid,
        }
    }

    fn domain(u: proto::friend::Friend) -> DomainFriend {
        DomainFriend {
            id: if u.id == 0 { None } else { Some(u.id) },
            uuid: if u.uuid.is_empty() { None } else { Some(u.uuid) },
            person_id: u.person_id,
            friend_id: u.friend_id,
            person_uuid: u.person_uuid,
            friend_uuid: u.friend_uuid,
            created_at: None,
            updated_at: None,
            status: business::domain::friend::FriendStatus::Pending,
        }
    }
}

// ---------------------------------------------------------------------------
// CountryMapper
// ---------------------------------------------------------------------------

impl Mapper<DomainCountry, proto::country::Country> for CountryMapper {
    fn response(t: DomainCountry) -> proto::country::Country {
        proto::country::Country {
            id: t.id,
            name: t.name,
            acronym: t.acronym,
            currency: t.currency,
        }
    }

    fn domain(u: proto::country::Country) -> DomainCountry {
        DomainCountry {
            id: u.id,
            name: u.name,
            acronym: u.acronym,
            currency: u.currency,
        }
    }
}

// ---------------------------------------------------------------------------
// ExerciseMapper
// ---------------------------------------------------------------------------

impl Mapper<DomainExercise, proto::exercise::Exercise> for ExerciseMapper {
    fn response(t: DomainExercise) -> proto::exercise::Exercise {
        proto::exercise::Exercise {
            id: t.id.unwrap_or(0),
            name: t.name,
            owner_id: t.owner_id,
            owner_uuid: t.owner_uuid,
            owner_name: t.owner_name,
            description: t.description.unwrap_or_default(),
            sets: t.sets,
            category: t.category.to_text(),
            reps_or_duration: t.reps_or_duration,
            uuid: t.uuid.unwrap_or_default(),
            visibility: t.visibility.to_string(),
            created_at: t.created_at.map(|d| d.to_string()).unwrap_or_default(),
            updated_at: t.updated_at.map(|d| d.to_string()).unwrap_or_default(),
        }
    }

    fn domain(u: proto::exercise::Exercise) -> DomainExercise {
        use business::domain::enums::{Category, Visibility};
        let id = if u.id == 0 { None } else { Some(u.id) };
        let uuid = if u.uuid.is_empty() { None } else { Some(u.uuid) };
        DomainExercise {
            id,
            uuid,
            name: u.name,
            description: Some(u.description),
            sets: u.sets,
            owner_id: u.owner_id,
            owner_uuid: u.owner_uuid,
            owner_name: u.owner_name,
            category: Category::from_string(&u.category),
            reps_or_duration: u.reps_or_duration,
            visibility: Visibility::from_string(&u.visibility),
            created_at: NaiveDateTime::parse_from_str(&u.created_at, "%Y-%m-%d %H:%M:%S%.f").ok(),
            updated_at: NaiveDateTime::parse_from_str(&u.updated_at, "%Y-%m-%d %H:%M:%S%.f").ok(),
        }
    }
}

// ---------------------------------------------------------------------------
// WorkoutMapper
// ---------------------------------------------------------------------------

impl Mapper<DomainWorkout, proto::workout::Workout> for WorkoutMapper {
    fn response(t: DomainWorkout) -> proto::workout::Workout {
        proto::workout::Workout {
            id: t.id.unwrap_or(0),
            uuid: t.uuid.unwrap_or_default(),
            owner_id: t.owner_id,
            owner_uuid: t.owner_uuid,
            name: t.name,
            description: t.description.unwrap_or_default(),
            difficulty: t.difficulty,
            muscle_group: t.muscle_group,
            visibility: t.visibility.to_string(),
            exercises: ExerciseMapper::response_vec(t.exercises),
            created_at: t.created_at.map(|d| d.to_string()).unwrap_or_default(),
            updated_at: t.updated_at.map(|d| d.to_string()).unwrap_or_default(),
        }
    }

    fn domain(u: proto::workout::Workout) -> DomainWorkout {
        use business::domain::enums::Visibility;
        let id = if u.id == 0 { None } else { Some(u.id) };
        let uuid = if u.uuid.is_empty() { None } else { Some(u.uuid) };
        DomainWorkout {
            id,
            uuid,
            owner_id: u.owner_id,
            owner_uuid: u.owner_uuid,
            name: u.name,
            description: Some(u.description),
            difficulty: u.difficulty,
            muscle_group: u.muscle_group,
            exercises: ExerciseMapper::domain_vec(u.exercises),
            visibility: Visibility::from_string(&u.visibility),
            created_at: NaiveDateTime::parse_from_str(&u.created_at, "%Y-%m-%d %H:%M:%S%.f").ok(),
            updated_at: NaiveDateTime::parse_from_str(&u.updated_at, "%Y-%m-%d %H:%M:%S%.f").ok(),
        }
    }
}

// ---------------------------------------------------------------------------
// BusinessProfileAddressMapper
// ---------------------------------------------------------------------------

impl Mapper<DomainBusinessProfileAddress, proto::business_profile_address::BusinessProfileAddress>
    for BusinessProfileAddressMapper
{
    fn response(
        t: DomainBusinessProfileAddress,
    ) -> proto::business_profile_address::BusinessProfileAddress {
        proto::business_profile_address::BusinessProfileAddress {
            id: t.id.unwrap_or(0),
            uuid: t.uuid.unwrap_or_default(),
            business_profile_id: t.business_profile_id,
            address_line_1: t.address_line1,
            address_line_2: t.address_line2.unwrap_or_default(),
            administrative_area: t.administrative_area,
            locality: t.locality,
            postal_code: t.postal_code.unwrap_or_default(),
            country_code: t.country_code,
            latitude: t.latitude.unwrap_or(0.0),
            longitude: t.longitude.unwrap_or(0.0),
            created_at: t.created_at.map(|d| d.to_string()).unwrap_or_default(),
            updated_at: t.updated_at.map(|d| d.to_string()).unwrap_or_default(),
        }
    }

    fn domain(
        u: proto::business_profile_address::BusinessProfileAddress,
    ) -> DomainBusinessProfileAddress {
        let id = if u.id == 0 { None } else { Some(u.id) };
        let uuid = if u.uuid.is_empty() { None } else { Some(u.uuid) };
        DomainBusinessProfileAddress {
            id,
            uuid,
            business_profile_id: u.business_profile_id,
            address_line1: u.address_line_1,
            address_line2: Some(u.address_line_2),
            locality: u.locality,
            administrative_area: u.administrative_area,
            postal_code: Some(u.postal_code),
            country_code: u.country_code,
            latitude: Some(u.latitude),
            longitude: Some(u.longitude),
            created_at: NaiveDateTime::parse_from_str(&u.created_at, "%Y-%m-%d %H:%M:%S%.f").ok(),
            updated_at: NaiveDateTime::parse_from_str(&u.updated_at, "%Y-%m-%d %H:%M:%S%.f").ok(),
        }
    }
}

// ---------------------------------------------------------------------------
// BusinessProfileMapper
// ---------------------------------------------------------------------------

impl Mapper<DomainBusinessProfile, proto::business_profile::BusinessProfile>
    for BusinessProfileMapper
{
    fn response(t: DomainBusinessProfile) -> proto::business_profile::BusinessProfile {
        proto::business_profile::BusinessProfile {
            id: t.id.unwrap_or(0),
            uuid: t.uuid.unwrap_or_default(),
            owner_id: t.owner_id,
            owner_uuid: t.owner_uuid,
            tax_id: t.tax_id,
            business_name: t.business_name,
            business_type: t.business_type.to_text(),
            social_name: t.social_name.unwrap_or_default(),
            object_key: t.object_key.unwrap_or_default(),
            logo: t.logo.unwrap_or_default(),
            cover_image: t.cover_image.unwrap_or_default(),
            created_at: t.created_at.map(|d| d.to_string()).unwrap_or_default(),
            updated_at: t.updated_at.map(|d| d.to_string()).unwrap_or_default(),
            addresses: BusinessProfileAddressMapper::response_vec(t.addresses),
        }
    }

    fn domain(u: proto::business_profile::BusinessProfile) -> DomainBusinessProfile {
        let id = if u.id == 0 { None } else { Some(u.id) };
        DomainBusinessProfile {
            id,
            uuid: Some(u.uuid),
            owner_id: u.owner_id,
            owner_uuid: u.owner_uuid,
            tax_id: u.tax_id,
            business_name: u.business_name,
            business_type: ProfileType::from_string(&u.business_type),
            social_name: Some(u.social_name),
            object_key: Some(u.object_key),
            logo: Some(u.logo),
            cover_image: Some(u.cover_image),
            created_at: NaiveDateTime::parse_from_str(&u.created_at, "%Y-%m-%d %H:%M:%S%.f").ok(),
            updated_at: NaiveDateTime::parse_from_str(&u.updated_at, "%Y-%m-%d %H:%M:%S%.f").ok(),
            addresses: BusinessProfileAddressMapper::domain_vec(u.addresses),
        }
    }
}

// ---------------------------------------------------------------------------
// SettingsMapper
// ---------------------------------------------------------------------------
impl Mapper<DomainSettings, proto::settings::Setting> for SettingsMapper {
    fn response(t: DomainSettings) -> proto::settings::Setting {
        proto::settings::Setting {
            id: t.id.unwrap_or(0),
            uuid: t.uuid.unwrap_or_default(),
            owner_id: t.person_id,
            owner_uuid: t.person_uuid,
            language: t.language,
            theme: t.theme,
            notifications_enabled: t.notifications_enabled,
            context_menu_position: t.context_menu_position.to_text(),
            home_page: t.home_page,
            created_at: t.created_at.map(|d| d.to_string()).unwrap_or_default(),
            updated_at: t.updated_at.map(|d| d.to_string()).unwrap_or_default(),
        }
    }

    fn domain(u: proto::settings::Setting) -> DomainSettings {
        let id = if u.id == 0 { None } else { Some(u.id) };
        DomainSettings {
            id,
            uuid: Some(u.uuid),
            person_id: u.owner_id,
            person_uuid: u.owner_uuid,
            language: u.language,
            theme: u.theme,
            notifications_enabled: u.notifications_enabled,
            context_menu_position: Position::from_string(&u.context_menu_position),
            home_page: u.home_page,
            created_at: NaiveDateTime::parse_from_str(&u.created_at, "%Y-%m-%d %H:%M:%S%.f").ok(),
            updated_at: NaiveDateTime::parse_from_str(&u.updated_at, "%Y-%m-%d %H:%M:%S%.f").ok(),
        }
    }
}
// ---------------------------------------------------------------------------
// TeamMemberMapper
// ---------------------------------------------------------------------------

impl Mapper<DomainTeamMember, proto::team_member::TeamMember> for TeamMemberMapper {
    fn response(t: DomainTeamMember) -> proto::team_member::TeamMember {
        proto::team_member::TeamMember {
            id: t.id.unwrap_or(0),
            uuid: t.uuid.unwrap_or_default(),
            business_profile_id: t.business_profile_id,
            business_profile_uuid: t.business_profile_uuid,
            person_id: t.person_id,
            person_uuid: t.person_uuid,
            status: t.status.as_str(),
        }
    }

    fn domain(u: proto::team_member::TeamMember) -> DomainTeamMember {
        DomainTeamMember {
            id: if u.id == 0 { None } else { Some(u.id) },
            uuid: if u.uuid.is_empty() { None } else { Some(u.uuid) },
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
