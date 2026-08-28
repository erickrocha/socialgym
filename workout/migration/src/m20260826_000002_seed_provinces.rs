use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

struct CountrySeed {
	acronym: &'static str,
	provinces: &'static [(&'static str, &'static str)],
}

const SEEDS: &[CountrySeed] = &[
	CountrySeed {
		acronym: "BR",
		provinces: &[
			("Acre", "AC"),
			("Alagoas", "AL"),
			("Amapá", "AP"),
			("Amazonas", "AM"),
			("Bahia", "BA"),
			("Ceará", "CE"),
			("Distrito Federal", "DF"),
			("Espírito Santo", "ES"),
			("Goiás", "GO"),
			("Maranhão", "MA"),
			("Mato Grosso", "MT"),
			("Mato Grosso do Sul", "MS"),
			("Minas Gerais", "MG"),
			("Pará", "PA"),
			("Paraíba", "PB"),
			("Paraná", "PR"),
			("Pernambuco", "PE"),
			("Piauí", "PI"),
			("Rio de Janeiro", "RJ"),
			("Rio Grande do Norte", "RN"),
			("Rio Grande do Sul", "RS"),
			("Rondônia", "RO"),
			("Roraima", "RR"),
			("Santa Catarina", "SC"),
			("São Paulo", "SP"),
			("Sergipe", "SE"),
			("Tocantins", "TO"),
		],
	},
	CountrySeed {
		acronym: "US",
		provinces: &[
			("Alabama", "AL"),
			("Alaska", "AK"),
			("Arizona", "AZ"),
			("Arkansas", "AR"),
			("California", "CA"),
			("Colorado", "CO"),
			("Connecticut", "CT"),
			("Delaware", "DE"),
			("Florida", "FL"),
			("Georgia", "GA"),
			("Hawaii", "HI"),
			("Idaho", "ID"),
			("Illinois", "IL"),
			("Indiana", "IN"),
			("Iowa", "IA"),
			("Kansas", "KS"),
			("Kentucky", "KY"),
			("Louisiana", "LA"),
			("Maine", "ME"),
			("Maryland", "MD"),
			("Massachusetts", "MA"),
			("Michigan", "MI"),
			("Minnesota", "MN"),
			("Mississippi", "MS"),
			("Missouri", "MO"),
			("Montana", "MT"),
			("Nebraska", "NE"),
			("Nevada", "NV"),
			("New Hampshire", "NH"),
			("New Jersey", "NJ"),
			("New Mexico", "NM"),
			("New York", "NY"),
			("North Carolina", "NC"),
			("North Dakota", "ND"),
			("Ohio", "OH"),
			("Oklahoma", "OK"),
			("Oregon", "OR"),
			("Pennsylvania", "PA"),
			("Rhode Island", "RI"),
			("South Carolina", "SC"),
			("South Dakota", "SD"),
			("Tennessee", "TN"),
			("Texas", "TX"),
			("Utah", "UT"),
			("Vermont", "VT"),
			("Virginia", "VA"),
			("Washington", "WA"),
			("West Virginia", "WV"),
			("Wisconsin", "WI"),
			("Wyoming", "WY"),
		],
	},
	CountrySeed {
		acronym: "ES",
		provinces: &[
			("Andalucía", "AN"),
			("Aragón", "AR"),
			("Asturias", "AS"),
			("Islas Baleares", "IB"),
			("Canarias", "CN"),
			("Cantabria", "CB"),
			("Castilla y León", "CL"),
			("Castilla-La Mancha", "CM"),
			("Cataluña", "CT"),
			("Extremadura", "EX"),
			("Galicia", "GA"),
			("La Rioja", "RI"),
			("Comunidad de Madrid", "MD"),
			("Región de Murcia", "MC"),
			("Comunidad Foral de Navarra", "NA"),
			("País Vasco", "PV"),
			("Comunidad Valenciana", "VC"),
		],
	},
	CountrySeed {
		acronym: "FR",
		provinces: &[
			("Auvergne-Rhône-Alpes", "ARA"),
			("Bourgogne-Franche-Comté", "BFC"),
			("Bretagne", "BRE"),
			("Centre-Val de Loire", "CVL"),
			("Corse", "COR"),
			("Grand Est", "GES"),
			("Hauts-de-France", "HDF"),
			("Île-de-France", "IDF"),
			("Normandie", "NOR"),
			("Nouvelle-Aquitaine", "NAQ"),
			("Occitanie", "OCC"),
			("Pays de la Loire", "PDL"),
			("Provence-Alpes-Côte d'Azur", "PAC"),
		],
	},
	CountrySeed {
		acronym: "DE",
		provinces: &[
			("Baden-Württemberg", "BW"),
			("Bayern", "BY"),
			("Berlin", "BE"),
			("Brandenburg", "BB"),
			("Bremen", "HB"),
			("Hamburg", "HH"),
			("Hessen", "HE"),
			("Mecklenburg-Vorpommern", "MV"),
			("Niedersachsen", "NI"),
			("Nordrhein-Westfalen", "NW"),
			("Rheinland-Pfalz", "RP"),
			("Saarland", "SL"),
			("Sachsen", "SN"),
			("Sachsen-Anhalt", "ST"),
			("Schleswig-Holstein", "SH"),
			("Thüringen", "TH"),
		],
	},
];

fn escape(value: &str) -> String {
	value.replace('\'', "''")
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let db = manager.get_connection();
		for seed in SEEDS {
			let values: Vec<String> = seed
				.provinces
				.iter()
				.map(|(name, acronym)| format!("('{}', '{}')", escape(name), escape(acronym)))
				.collect();
			let sql = format!(
				"INSERT INTO province (name, acronym, country_id) \
				 SELECT v.name, v.acronym, country.id \
				 FROM (VALUES {}) AS v(name, acronym) \
				 CROSS JOIN country WHERE country.acronym = '{}'",
				values.join(", "),
				escape(seed.acronym)
			);
			db.execute_unprepared(&sql).await?;
		}
		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let db = manager.get_connection();
		let acronyms: Vec<String> = SEEDS
			.iter()
			.map(|seed| format!("'{}'", escape(seed.acronym)))
			.collect();
		let sql = format!(
			"DELETE FROM province WHERE country_id IN (SELECT id FROM country WHERE acronym IN ({}))",
			acronyms.join(", ")
		);
		db.execute_unprepared(&sql).await?;
		Ok(())
	}
}
