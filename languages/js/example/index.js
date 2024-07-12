const { SdkrsClient: SdkrsClientWasm, LogLevel } = require("@sdkrs/sdk-wasm");
const sdk = require("@sdkrs/sdk-client");

async function main() {
  const settings = {
    apiUrl: process.env.API_URL,
    identityUrl: process.env.IDENTITY_URL,
  };

  const client = new sdk.SdkrsClient(
    new SdkrsClientWasm(JSON.stringify(settings), LogLevel.Debug),
  );

  const organization_id = process.env.ORGANIZATION_ID;
  await client.accessTokenLogin(process.env.ACCESS_TOKEN);

  const project = await client.projects().create("test", organization_id);
  const projects = await client.projects().list(organization_id);
  console.log(projects.data);

  const secret = await client
    .secrets()
    .create("test-secret", "My secret!", "This is my secret", [project.id], organization_id);
  const secrets = await client.secrets().list(organization_id);
  console.log(secrets.data);

  console.log(project, secret);

  await client.projects().delete([project.id]);
  await client.secrets().delete([secret.id]);
}
main();
