variable "server_image" {
  description = "Docker image for the server"
  default     = "zkp_server"
}

variable "client_image" {
  description = "Docker image for the client"
  default     = "zkp_client"
}

variable "ZKP_REPO_URL" {
  description = "URL of the repository"
  type        = string
}

variable "AWS_ACCESS_KEY_ID" {
  description = "AWS Access Key ID"
  type        = string
}

variable "AWS_SECRET_ACCESS_KEY" {
  description = "AWS Secret Access Key"
  type        = string
}

variable "AWS_REGION" {
  description = "AWS Region"
  type        = string
}