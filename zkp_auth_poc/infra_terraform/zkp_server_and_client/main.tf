variable "ssh_private_key_path" {
  description = "Path to the private SSH key"
  default     = "~/.ssh/id_zkp_terraform"
}

variable "ssh_public_key_path" {
  description = "Path to the public SSH key"
  default     = "~/.ssh/id_zkp_terraform.pub"
}

provider "aws" {
  region     = var.AWS_REGION
  access_key = var.AWS_ACCESS_KEY_ID
  secret_key = var.AWS_SECRET_ACCESS_KEY
}

resource "aws_key_pair" "zkp_key" {
  key_name   = "zkp_key"
  public_key = file(var.ssh_public_key_path)
}

resource "aws_vpc" "zkp_vpc" {
  cidr_block = "10.0.0.0/16"
}

resource "aws_subnet" "zkp_subnet" {
  vpc_id                  = aws_vpc.zkp_vpc.id
  cidr_block              = "10.0.1.0/24"
  map_public_ip_on_launch = true
}

resource "aws_internet_gateway" "zkp_igw" {
  vpc_id = aws_vpc.zkp_vpc.id
}

resource "aws_route_table" "zkp_route_table" {
  vpc_id = aws_vpc.zkp_vpc.id

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.zkp_igw.id
  }
}

resource "aws_route_table_association" "zkp_route_table_association" {
  subnet_id      = aws_subnet.zkp_subnet.id
  route_table_id = aws_route_table.zkp_route_table.id
}

resource "aws_security_group" "zkp_sg" {
  vpc_id = aws_vpc.zkp_vpc.id

  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 50051
    to_port     = 50051
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = -1
    to_port     = -1
    protocol    = "icmp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

data "template_file" "cloud_init_server" {
  template = file("${path.module}/cloud-init-server.yaml")
  vars = {
    aws_access_key_id     = var.AWS_ACCESS_KEY_ID
    aws_secret_access_key = var.AWS_SECRET_ACCESS_KEY
    aws_region            = var.AWS_REGION
    repo_url              = var.ZKP_REPO_URL
    ssh_pub               = file(var.ssh_public_key_path)
  }
}

resource "aws_instance" "zkp_server" {
  ami                         = "ami-0e04bcbe83a83792e" # Ubuntu 24.04
  instance_type               = "t2.micro"
  subnet_id                   = aws_subnet.zkp_subnet.id
  vpc_security_group_ids      = [aws_security_group.zkp_sg.id]
  key_name                    = aws_key_pair.zkp_key.key_name
  associate_public_ip_address = true
  user_data                   = data.template_file.cloud_init_server.rendered
  tags = {
    Name = "zkp-server"
  }
}

data "template_file" "cloud_init_client" {
  template = file("${path.module}/cloud-init-client.yaml")
  vars = {
    aws_access_key_id     = var.AWS_ACCESS_KEY_ID
    aws_secret_access_key = var.AWS_SECRET_ACCESS_KEY
    aws_region            = var.AWS_REGION
    repo_url              = var.ZKP_REPO_URL
    ssh_pub               = file(var.ssh_public_key_path)
    server_ip             = aws_instance.zkp_server.public_ip
  }
}

resource "aws_instance" "zkp_client" {
  ami                         = "ami-0e04bcbe83a83792e" # Ubuntu 24.04
  instance_type               = "t2.micro"
  subnet_id                   = aws_subnet.zkp_subnet.id
  vpc_security_group_ids      = [aws_security_group.zkp_sg.id]
  key_name                    = aws_key_pair.zkp_key.key_name
  associate_public_ip_address = true
  user_data                   = data.template_file.cloud_init_client.rendered
  tags = {
    Name = "zkp-client"
  }
  depends_on = [ aws_instance.zkp_server ]
}

output "zkp_server_public_ip" {
  value = aws_instance.zkp_server.public_ip
}

output "zkp_client_public_ip" {
  value = aws_instance.zkp_client.public_ip
}
