use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Text;
use rocket::response::{status::Created, Debug};
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post, delete};
use rocket_dyn_templates::{context, Template};
use rocket_sync_db_pools::diesel;
use rocket::response::status::{BadRequest, NotFound};
use crate::db::*;
use crate::gemini::{PromptPayload, get_gemini_api_key, generate as generate_cotizacion};
use crate::personas::QUOTER;
use crate::claims::AppState;
use chrono::prelude::*;
use chrono::NaiveDate;

/* TODO: use crate::claims::{AppUser, Claims};
use crate::models::Cotizaciones;
use crate::schema::users::dsl::*;
use diesel::result::Error;
//use std::error::Error;
*/
type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;


/// Show the cotizaciones landing page
#[get("/")]
pub async fn index(pdb: PajaDB) -> Template {
    /*let results =
    pdb.run(move |connection| 
        crate::schema::cotizaciones::dsl::cotizaciones
            .load::<User>(connection)
            .expect("Error loading cotizaciones")
    ).await;*/
    Template::render("cotizaciones", context! {id: 1})
}


/// Show the cotizaciones landing page
#[post("/generate", format = "json", data = "<user_data>")]
pub async fn generate(user_data: Json<PromptPayload>) -> Json<String> {
    /*let results =
    pdb.run(move |connection| 
        crate::schema::cotizaciones::dsl::cotizaciones
            .load::<User>(connection)
            .expect("Error loading cotizaciones")
    ).await;*/


    let today = Utc::now().naive_utc();

    //let output = generate_cotizacion(&QUOTER, format!(r#"Esta es tu personalidad: {}, Este es el contexto que te
    let output = generate_cotizacion(&QUOTER, format!(r#"En este contexto te
        estoy proporcionando los rubros que tienes que usar para generar
        una cotización agrega el desglose del IVA mexicano que es del 16%.
        Contexto en json: {}, la fecha de hoy es: {}". La salida debe estar en
        HTML basado en el JSON de system_instructions"#, user_data.content, today)
        .into(), get_gemini_api_key()).await.unwrap();


    //let output = String::from("¡Claro! Aquí tienes una cotización generada con el rubro proporcionado y el desglose del IVA mexicano del 16%.\n\n---\n\n**COTIZACIÓN DE SERVICIOS DE MANTENIMIENTO**\n\n**Fecha:** 26 de octubre de 2023\n**Número de Cotización:** Q-001-2023\n**Válido hasta:** 25 de noviembre de 2023\n\n**Para:** [Nombre del Cliente/Empresa]\n**Atención:** [Nombre del Contacto]\n\n**Estimado(a) Cliente/Empresa,**\n\nNos complace presentarle la siguiente cotización por los servicios de mantenimiento mensual, de acuerdo a sus requerimientos.\n\n---\n\n**DESGLOSE DE SERVICIOS Y COSTOS**\n\n| Concepto             | Cantidad | Precio Unitario (MXN) | Subtotal (MXN) |\n| :------------------- | :------- | :-------------------- | :------------- |\n| Mantenimiento Mensual | 1 mes    | $200,000.00           | $200,000.00    |\n| **SUBTOTAL**         |          |                       | **$200,000.00**|\n| **IVA (16%)**        |          |                       | **$32,000.00** |\n| **TOTAL A PAGAR**    |          |                       | **$232,000.00**|\n\n---\n\n**Notas Importantes:**\n\n*   **Vigencia:** Esta cotización es válida por 30 días naturales a partir de la fecha de emisión.\n*   **Moneda:** Todos los precios están expresados en Pesos Mexicanos (MXN).\n*   **Forma de Pago:** [Especificar, ej: Transferencia bancaria, depósito en cuenta No. XXXX]\n*   **Condiciones de Pago:** [Especificar, ej: Pago de contado al inicio del servicio, Neto 30 días]\n*   **Servicios Incluidos:** [Aquí se detallarían los servicios específicos que cubre el mantenimiento mensual, ej: revisión de equipos, soporte técnico, etc.]\n\nAgradecemos su interés y quedamos a su disposición para cualquier consulta adicional o aclaración.\n\nAtentamente,\n\n[Tu Nombre/Nombre de la Empresa]\n[Tu Cargo/Departamento]\n[Tu Teléfono]\n[Tu Correo Electrónico]\n[Tu Página Web (opcional)]\n\n---");
/*    let output = String::from(r#"¡Claro! Aquí tienes una cotización detallada para los servicios solicitados, incluyendo el desglose del IVA mexicano del 16%. --- **[Nombre de tu Empresa]** [Dirección de tu Empresa] [Teléfono] | [Correo Electrónico] | [Sitio Web] --- **COTIZACIÓN DE SERVICIOS - Implementación y Mantenimiento** **Fecha:** 05 de septiembre de 2025 **Folio:** COT-NCO-2025-09-001 **A la atención de:** [Nombre del Cliente / Departamento] **Empresa:** [Nombre de la Empresa Cliente] **Asunto:** Propuesta de Servicios para Nextcloud Administrado sobre OpenStack Estimados, Por medio de la presente, nos complace presentar la cotización para la implementación y administración de un entorno Nextcloud sobre un Tenant de OpenStack con recursos suficientes, así como los servicios de capacitación y el mantenimiento mensual asociado. --- ### **1. Servicios de Implementación y Configuración Inicial** Este rubro cubre la totalidad de la fase de puesta en marcha, configuración, optimización y capacitación, sumando el total de $3,500,000 MXN antes de impuestos. | Rubro | Descripción del Servicio | Monto (MXN, sin IVA) | | :---------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------ | | **Configuración e Implementación de Infraestructura y Plataforma** | Diseño, aprovisionamiento y configuración de un Tenant de OpenStack con recursos suficientes (cómputo, almacenamiento, red, seguridad).
Instalación, configuración y puesta en marcha de la plataforma Nextcloud administrada, incluyendo bases de datos, almacenamiento de archivos y optimización de rendimiento.
Integración con sistemas existentes (si aplica, como LDAP/AD).
Pruebas exhaustivas de funcionalidad, seguridad y rendimiento. | $2,100,000.00 | | **Capacitación y Documentación** | Desarrollo e impartición de sesiones de capacitación para administradores del sistema sobre la gestión del Tenant de OpenStack y la plataforma Nextcloud (gestión de usuarios, permisos, backups, monitoreo básico).
Capacitación para usuarios finales sobre el uso de las funcionalidades de Nextcloud (colaboración, compartición, sincronización).
Creación de manuales de usuario y administración personalizados. | $1,400,000.00 | | **SUBTOTAL Servicios Iniciales (sin IVA)** | | **$3,500,000.00** | | **IVA (16%)** | | **$560,000.00** | | **TOTAL Servicios Iniciales (con IVA)** | | **$4,060,000.00** | --- ### **2. Servicios de Mantenimiento y Soporte Mensual** Este rubro detalla el costo recurrente para asegurar la operatividad, seguridad y rendimiento óptimo de su entorno Nextcloud y OpenStack. | Rubro | Descripción del Servicio | Monto Mensual (MXN, sin IVA) | | :---------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------- | | **Mantenimiento Mensual del Entorno Nextcloud y OpenStack** | Monitoreo proactivo 24/7 de la infraestructura de OpenStack y la plataforma Nextcloud.
Aplicación de actualizaciones de seguridad, parches del sistema operativo y nuevas versiones de Nextcloud (previa coordinación).
Soporte técnico especializado y resolución de incidencias.
Gestión de copias de seguridad (backups) y planes de recuperación.
Optimización periódica de rendimiento y escalabilidad de recursos. | $200,000.00 | | **IVA (16%) Mensual** | | **$32,000.00** | | **TOTAL Mensual (con IVA)** | | **$232,000.00** | --- ### **Resumen General de Costos** | Concepto | Monto (MXN, sin IVA) | IVA (16%) | Total (MXN, con IVA) | | :-------------------------------------- | :-------------------- | :--------- | :-------------------- | | **Servicios de Implementación y Capacitación** | $3,500,000.00 | $560,000.00 | $4,060,000.00 | | **Mantenimiento y Soporte Mensual** | $200,000.00 | $32,000.00 | $232,000.00 | --- ### **Términos y Condiciones** * **Validez de la Oferta:** Esta cotización es válida por 30 días naturales a partir de la fecha de emisión. * **Moneda:** Todos los precios están expresados en Pesos Mexicanos (MXN). * **Condiciones de Pago (Servicios Iniciales):** * 50% al inicio del proyecto y firma del contrato. * 50% a la finalización y aceptación de los servicios de implementación y capacitación. * **Condiciones de Pago (Mantenimiento Mensual):** * El servicio de mantenimiento mensual comenzará una vez finalizada la fase de implementación inicial. * Pago anticipado los primeros 5 días hábiles de cada mes. * **Alcance:** Cualquier servicio o requerimiento adicional no especificado en esta propuesta será objeto de una cotización complementaria. Agradecemos su interés en nuestros servicios y quedamos a su disposición para cualquier aclaración o consulta. Atentamente, --- **[Tu Nombre/Nombre del Representante]** [Tu Cargo] [Nombre de tu Empresa] [Contacto]"#);
*/
    //Ok(Json(results))
    Json(output)
    //Template::render("cotizaciones", context! {id: 1})
}



/*

/// Creates a user
#[post("/add", format = "json", data = "<arg_user>")]
pub async fn add(arg_user: Json<User>, _user: Claims, tdb: ChacaDB) ->
Result<Created<Json<User>>, BadRequest<String>> {
    let user: User = arg_user.into_inner();
	let result: User = match tdb 
		.run(move |conn| { 
			sql_query("INSERT INTO users VALUES(gen_random_uuid(), $1, $2,
				get_pw_hash($3), $4, $5,
				$6, $7, now(), now(), now()) RETURNING *;")
				.bind::<Text, _>(user.name.expect("Error deserializing user name")) // TODO: send error if None
				.bind::<Text, _>(user.email)
				.bind::<Text, _>(user.password.expect("Error deserializing password"))
				.bind::<Text, _>(user.oauth_provider)
				.bind::<Text, _>(user.oauth_user_id)
				.bind::<Text, _>(user.access_token)
				.bind::<Text, _>(user.refresh_token.expect("Error deserializing refresh_token")) // TODO: send error if None
				.get_result(conn)
			}).await {

        //Ok(r) => r,
        Ok(r) => r,
        Err(e) => return Err(BadRequest(format!("Error creating user: {}", e).to_string())),
    };

    Ok(Created::new("/").body(Json(result)))
}

//https://api.rocket.rs/v0.5/rocket_sync_db_pools/

/// Get a user and returns it as a JSON object
#[get("/<userid>")]
pub async fn get(userid: Uuid, tdb: ChacaDB) ->
Result<Json<Vec<User>>, NotFound<String>> {
    let results = tdb.run(move |connection|
        crate::schema::users::dsl::users
            .filter(id.eq(userid))
            .load::<User>(connection)
        .expect("Error loading users")
    ).await;
    if results.len() > 0 {
        Ok(Json(results))
    } else {
        Err(NotFound(format!("Could not find user: {}", userid)))
    }
}

/// Get a user and returns it as a JSON object
#[get("/oauth2/<oauth_id>")]
pub async fn get_by_oauth(oauth_id: String, tdb: ChacaDB) ->
Result<Json<Vec<User>>, NotFound<String>> {
    //TODO: check borrows
    let my_id = oauth_id.clone();
    let results = tdb.run(move |connection|
        crate::schema::users::dsl::users
            .filter(oauth_user_id.eq(oauth_id.clone()))
            .load::<User>(connection)
        .expect("Error loading user")
    ).await;
    if results.len() > 0 {
        Ok(Json(results))
    } else {
        Err(NotFound(format!("Could not find user: {}", my_id)))
    }
}

/// Remove a user
#[delete("/<userid>")]
pub async fn delete(userid: Uuid, _user: Claims, tdb: ChacaDB) ->
Result<Json<String>, NotFound<String>> {
    let results = tdb.run(move |connection|
        diesel::delete(
            crate::schema::users::dsl::users
            .filter(id.eq(userid)))
            .execute(connection)
    ).await;
    if results.unwrap() == 1 {
        Ok(Json(format!("{} deleted", userid)))
    } else {
        Err(NotFound(format!("Could not find user: {}", userid)))
    }
}

/*TODO: update for new AppUser: get the user uuid from the database
#[get("/me")]
pub async fn me(claim: Claims) -> Json<AppUser> {
    Json(AppUser {
        id: claim.id,
        name: claim.name,
        email: claim.email,
    })
}
*/

/*******************************************************************************
*                                                                              *
*                                                                              *
*                      G E N E R A L  F U N C T I O N S                        *
*                                                                              *
*                                                                              *
********************************************************************************/

impl User {
    /// Creates a user
    pub async fn insert(
        self,
        cdb: ChacaDB
    ) -> Result<usize, Error> {
        cdb.run(move |conn| {
                diesel::insert_into(users).values(&self).execute(conn)
        }).await
    }

    /// Updates a user
    pub async fn update(
        self,
        cdb: &ChacaDB
    ) -> Result<usize, Error> {
        cdb.run(move |conn| {
            diesel::update(users)
            .set(self)
            .execute(conn)
        }).await
    }

    pub async fn load_by_oauth(
        oauth_id: String,
        cdb: &ChacaDB
    ) -> Result<Self, NotFound<String>> {
        //TODO: check borrows
        let my_id = oauth_id.clone();
        let results = cdb.run(move |connection|
            crate::schema::users::dsl::users
                .filter(oauth_user_id.eq(oauth_id.clone()))
                .load::<User>(connection)
            .expect("Error loading user")
        ).await;
        if results.len() > 0 {
            Ok(results[0].clone())
        } else {
            Err(NotFound(format!("Could not find user: {}", my_id)))
        }
    }

}
*/
