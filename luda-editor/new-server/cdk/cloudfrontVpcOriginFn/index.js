const {
    CloudFrontClient,
    ListVpcOriginsCommand,
    DeleteVpcOriginCommand,
    GetVpcOriginCommand,
} = require("@aws-sdk/client-cloudfront");

exports.handler = async (event, context) => {
    const client = new CloudFrontClient({});

    switch (event.RequestType) {
        case "Create":
            // nothing to do
            break;
        case "Update":
            // nothing to do
            break;
        case "Delete":
            const response = await client.send(new ListVpcOriginsCommand({}));
            const items = response.VpcOriginList.Items.filter((item) => {
                item.Name === event.ResourceProperties.vpcOriginName;
            });
            for (const item of items) {
                const response = await client.send(
                    new GetVpcOriginCommand({ Id: item.Id }),
                );
                await client.send(
                    new DeleteVpcOriginCommand({
                        Id: item.Id,
                        IfMatch: response.ETag,
                    }),
                );
            }
            break;
    }
};
