// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/rusoto/rusoto/master/assets/logo-square.png"
)]
//! <p>You can use the Amazon Elastic Block Store (EBS) direct APIs to directly read the data on your EBS snapshots, and identify the difference between two snapshots. You can view the details of blocks in an EBS snapshot, compare the block difference between two snapshots, and directly access the data in a snapshot. If you’re an independent software vendor (ISV) who offers backup services for EBS, the EBS direct APIs makes it easier and more cost-effective to track incremental changes on your EBS volumes via EBS snapshots. This can be done without having to create new volumes from EBS snapshots, and then use EC2 instances to compare the differences.</p> <p>This API reference provides detailed information about the actions, data types, parameters, and errors of the EBS direct APIs. For more information about the elements that make up the EBS direct APIs, and examples of how to use them effectively, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-accessing-snapshot.html">Accessing the Contents of an EBS Snapshot</a>. For more information about how to use the EBS direct APIs, see the <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-accessing-snapshots.html">EBS direct APIs User Guide</a>. To view the currently supported AWS Regions and endpoints for the EBS direct APIs, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html#ebs_region">AWS Service Endpoints</a> in the <i>AWS General Reference</i>.</p>
//!
//! If you're using the service, you're probably looking for [EbsClient](struct.EbsClient.html) and [Ebs](trait.Ebs.html).

mod custom;
mod generated;
pub use custom::*;
pub use generated::*;
