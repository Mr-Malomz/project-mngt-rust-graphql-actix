use dotenv::dotenv;
use futures::TryStreamExt;
use std::{env, io::Error};

use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection, Database,
};

use crate::models::project_model::{Owner, Project};

pub struct MongoDB {
    db: Database,
}

impl MongoDB {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let db = client.database("projectMngt");
        MongoDB { db }
    }

    fn colHelper<T>(dataSource: &Self, collectionName: &str) -> Collection<T> {
        dataSource.db.collection(collectionName)
    }

    //Owners logic
    pub async fn create_owner(&self, new_owner: Owner) -> Result<Owner, Error> {
        let new_doc = Owner {
            _id: None,
            name: new_owner.name.clone(),
            email: new_owner.email.clone(),
            phone: new_owner.phone.clone(),
        };

        let col = MongoDB::colHelper::<Owner>(&self, "owner");

        let data = col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating owner");

        let new_owner = Owner {
            _id: data.inserted_id.as_object_id(),
            name: new_owner.name.clone(),
            email: new_owner.email.clone(),
            phone: new_owner.phone.clone(),
        };

        Ok(new_owner)
    }

    async fn get_owners(&self) -> Result<Vec<Owner>, Error> {
        let col = MongoDB::colHelper::<Owner>(&self, "owner");

        let mut cursors = col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of owners");

        let mut owners: Vec<Owner> = Vec::new();
        while let Some(owner) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            owners.push(owner)
        }

        Ok(owners)
    }

    async fn single_owner(&self, id: &String) -> Result<Owner, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};

        let col = MongoDB::colHelper::<Owner>(&self, "owner");

        let owner_detail = col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting owner's detail");

        Ok(owner_detail.unwrap())
    }

    //project logics
    pub async fn create_project(&self, new_project: Project) -> Result<Project, Error> {
        let new_doc = Project {
            _id: None,
            owner_id: new_project.owner_id.clone(),
            name: new_project.name.clone(),
            description: new_project.description.clone(),
            status: new_project.status.clone(),
        };

        let col = MongoDB::colHelper::<Project>(&self, "project");

        let data = col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating project");

        let new_project = Project {
            _id: data.inserted_id.as_object_id(),
            owner_id: new_project.owner_id.clone(),
            name: new_project.name.clone(),
            description: new_project.description.clone(),
            status: new_project.status.clone(),
        };

        Ok(new_project)
    }

    async fn get_projects(&self) -> Result<Vec<Project>, Error> {
        let col = MongoDB::colHelper::<Project>(&self, "project");

        let mut cursors = col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of projects");

        let mut projects: Vec<Project> = Vec::new();
        while let Some(project) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            projects.push(project)
        }

        Ok(projects)
    }

    async fn single_project(&self, id: &String) -> Result<Project, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};

        let col = MongoDB::colHelper::<Project>(&self, "project");

        let project_detail = col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting project's detail");

        Ok(project_detail.unwrap())
    }
}
