use anyhow::Result;
use clap::Parser;
use guppy::{MetadataCommand, graph::PackageGraph};
use inquire::Select;
use regex::Regex;
use strum::IntoEnumIterator;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Access the version of the document you are using"
    )]
    sync: bool,
}

#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter)]
enum Service {
    Amplify,
    APIGateway,
    APIGatewayV2,
    AppFlow,
    AppMesh,
    AppRunner,
    AppSync,
    Athena,
    Batch,
    Bedrock,
    Billing,
    Budgets,
    Chatbot,
    Cloud9,
    CloudFormation,
    CloudFront,
    CloudTrail,
    CloudWatch,
    CodeBuild,
    CodeCatalyst,
    CodeCommit,
    CodeDeploy,
    CodePipeline,
    Comprehend,
    ComprehendMedical,
    ControlTower,
    DataZone,
    DynamoDB,
    EBS,
    EC2,
    ECR,
    ECS,
    EFS,
    EKS,
    ElasticBeanstalk,
    ElasticLoadBalancing,
    ElasticLoadBalancingV2,
    EMR,
    EventBridge,
    EventBridgePipes,
    EventBridgeScheduler,
    Firehose,
    Glue,
    GlueDataBrew,
    GuardDuty,
    IAM,
    IdentityStore,
    IoTGreenGrass,
    IoTGreenGrassV2,
    Lambda,
    QuickSight,
    RAM,
    RDS,
    RedShift,
    RedShiftData,
    RedShiftServerless,
    S3,
    S3Glacier,
    S3Tables,
    SageMaker,
    SecretsManager,
    SES,
    SQS,
    StepFunctions,
    SNS,
    SSO,
    STS,
    UserNotifications,
}

impl Service {
    fn url_expression(&self) -> &str {
        match self {
            Service::Amplify => "amplify",
            Service::APIGateway => "apigateway",
            Service::APIGatewayV2 => "apigatewayv2",
            Service::AppFlow => "appflow",
            Service::AppMesh => "appmesh",
            Service::AppRunner => "apprunner",
            Service::AppSync => "appsync",
            Service::Athena => "athena",
            Service::Batch => "batch",
            Service::Bedrock => "bedrock",
            Service::Billing => "billing",
            Service::Budgets => "budgets",
            Service::Chatbot => "chatbot",
            Service::Cloud9 => "cloud9",
            Service::CloudFormation => "cloudformation",
            Service::CloudFront => "cloudfront",
            Service::CloudTrail => "cloudtrail",
            Service::CloudWatch => "cloudwatch",
            Service::CodeBuild => "codebuild",
            Service::CodeCatalyst => "codecatalyst",
            Service::CodeCommit => "codecommit",
            Service::CodeDeploy => "codedeploy",
            Service::CodePipeline => "codepipeline",
            Service::Comprehend => "comprehend",
            Service::ComprehendMedical => "comprehendmedical",
            Service::ControlTower => "controltower",
            Service::DataZone => "datazone",
            Service::DynamoDB => "dynamodb",
            Service::EBS => "ebs",
            Service::EC2 => "ec2",
            Service::ECR => "ecr",
            Service::ECS => "ecs",
            Service::EFS => "efs",
            Service::EKS => "eks",
            Service::ElasticBeanstalk => "elasticbeanstalk",
            Service::ElasticLoadBalancing => "elasticloadbalancing",
            Service::ElasticLoadBalancingV2 => "elasticloadbalancingv2",
            Service::EMR => "emr",
            Service::EventBridge => "eventbridge",
            Service::EventBridgePipes => "pipes",
            Service::EventBridgeScheduler => "scheduler",
            Service::Firehose => "firehose",
            Service::Glue => "glue",
            Service::GlueDataBrew => "databrew",
            Service::GuardDuty => "guardduty",
            Service::IAM => "iam",
            Service::IdentityStore => "identitystore",
            Service::IoTGreenGrass => "greengrass",
            Service::IoTGreenGrassV2 => "greengrassv2",
            Service::Lambda => "lambda",
            Service::QuickSight => "quicksight",
            Service::RAM => "ram",
            Service::RDS => "rds",
            Service::RedShift => "redshift",
            Service::RedShiftData => "redshiftdata",
            Service::RedShiftServerless => "redshiftserverless",
            Service::S3 => "s3",
            Service::S3Glacier => "glacier",
            Service::S3Tables => "s3tables",
            Service::SageMaker => "sagemaker",
            Service::SecretsManager => "secretsmanager",
            Service::SES => "ses",
            Service::SQS => "sqs",
            Service::StepFunctions => "sfn",
            Service::SNS => "sns",
            Service::SSO => "sso",
            Service::STS => "sts",
            Service::UserNotifications => "notifications",
        }
    }

    fn regex_packge_id(&self) -> Regex {
        let mut pattern = format!("aws-sdk-{}", self.url_expression());
        pattern.push_str(r#"@(?P<major>[0-9]+)\.(?P<minor>[0-9]+)\.(?P<patch>[0-9]+)$"#);
        Regex::new(pattern.as_str()).unwrap()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let all_services = Service::iter().collect::<Vec<_>>();
    let s = Select::new(
        "Which Client documents do you want to access?",
        all_services,
    )
    .prompt()?;

    let version = if cli.sync {
        let mut cmd = MetadataCommand::new();
        let package_graph = PackageGraph::from_command(&mut cmd)?;
        let re = s.regex_packge_id();
        let mut version = "latest".to_string();
        for id in package_graph.package_ids() {
            let id = id.repr();
            if let Some(caps) = re.captures(id) {
                let major = caps.name("major").unwrap().as_str();
                let minor = caps.name("minor").unwrap().as_str();
                let patch = caps.name("patch").unwrap().as_str();
                version = format!("{major}.{minor}.{patch}");
                break;
            }
        }
        version
    } else {
        "latest".to_string()
    };
    let s = s.url_expression();
    let url =
        format!("https://docs.rs/aws-sdk-{s}/{version}/aws_sdk_{s}/client/struct.Client.html");
    open::that(url)?;

    Ok(())
}
