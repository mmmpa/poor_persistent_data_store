provider "aws" {
  region = "us-west-2"
  access_key = "fakeMyKeyId"
  secret_key = "fakeSecretAccessKey"
  skip_credentials_validation = true
  skip_metadata_api_check = true
  skip_requesting_account_id = true

  endpoints {
    dynamodb = "http://localhost:8000"
  }
}

module "db" {
  source = "../.mods"

  stage = "test"
}
