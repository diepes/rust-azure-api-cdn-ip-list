# Retrieve info from Azure for Verizon CDN

Log into Azure using the raw api ```https://login.microsoftonline.com/{}/oauth2/token\``` passing required headers to get Oauth token.
Then retrieve CDNS ip's from ```"https://management.azure.com/providers/Microsoft.Cdn/edgenodes?api-version=2023-05-01"```

## Set login credentials in .env

* Create App in Azure Id
  * Create Client_ID (AppID) and secret for the app to login with

* Create .env file in root of repo

      ## NOT in GIT##
      TENANT_ID="39......-....-....-....-............3"
      # client id / app id
      CLIENT_ID="4.......-....-....-....-...........5"
      # client secret / password for app/client id
      CLIENT_SECRET="F......................................D"

## High level flow

1. src/req_get_info.rs gets Oauth token from [https://login.microsoftonline.com/{TENANT_ID}/oauth2/token]
2. src/req_get_info.rs then gets Verizon cdn ips from [https://management.azure.com/providers/Microsoft.Cdn/edgenodes?api-version=2023-05-01]
3. src/parse_cdn.rs with serde is used to parse the info
4. we grab all the ipv4 ips and masks, make them uniq and print them in a list that can be used with nginx real_ip module, to identify valid forwarded_for IP's
