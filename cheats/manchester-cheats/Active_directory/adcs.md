# ADCS tips

## Certficate + ENROLEE_SUPLLIES_SUBJECT = WSUS-Server-Trust Anchor
```
# Certficate + ENROLEE_SUPLLIES_SUBJECT = WSUS-Server-Trust Anchor
if the template is not a PKINIT primitive & Server Authentication EKU won't do Kerberos — may be an opportunity to impersonate any HTTPS service the DC trusts, including its own WSUS endpoint.
```
