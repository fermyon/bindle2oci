# Spin bindle2oci plugin

Migrate Spin applications from Bindle to an OCI registry.

## Build from source

You will need Rust and the `pluginify` plugin (`spin plugins install --url https://github.com/itowlson/spin-pluginify/releases/download/canary/pluginify.json`).

```
cargo build --release
spin pluginify --install
```

## Registry authentication

Before you can push to a registry, you need to log in to the registry.

```bash
$ spin registry login --username <username> --password <password>
```

For more details on authenticating to a registry, refer to the [logging into a registry](https://developer.fermyon.com/spin/distributing-apps#logging-into-a-registry) guide in the documentation.

## Migrate an application from a Bindle server to an OCI registry

```bash
spin bindle2oci \
    --bindle-server <server> \
    --bindle-username <username> \
    --bindle-password <password> \
    --bindle <name> \
    --reference <new-reference>
```
