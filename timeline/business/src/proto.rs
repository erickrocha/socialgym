pub mod proto {

    pub mod business_profile {
        tonic::include_proto!("grpc.business_profile");
    }

    pub mod business_profile_address {
        tonic::include_proto!("grpc.business_profile_address");
    }

    pub mod country {
        tonic::include_proto!("grpc.country");
    }

    pub mod exercise {
        tonic::include_proto!("grpc.exercise");
    }

    pub mod friend {
        tonic::include_proto!("grpc.friend");
    }

    pub mod person {
        tonic::include_proto!("grpc.person");
    }

    pub mod person_address {
        tonic::include_proto!("grpc.person_address");
    }

    pub mod person_info {
        tonic::include_proto!("grpc.person_info");
    }

    pub mod team_member {
        tonic::include_proto!("grpc.team_member");
    }

    pub mod settings {
        tonic::include_proto!("grpc.settings");
    }
    pub mod user {
        tonic::include_proto!("grpc.user");
    }

    pub mod workout {
        tonic::include_proto!("grpc.workout");
    }

    pub mod resource {
        tonic::include_proto!("grpc.resource");
    }
}