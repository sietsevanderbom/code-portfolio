provider "aws" {
  region = var.AWS_REGION
}

variable "AWS_REGION" {
  description = "AWS Region"
}

resource "aws_ecr_repository" "zkp_repo" {
  name = "zkp-repo"
  force_delete = true
}

output "zkp_repo_url" {
  value = aws_ecr_repository.zkp_repo.repository_url
}