# \Oauth2ProviderApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tenant_oauth2_provider**](Oauth2ProviderApi.md#create_tenant_oauth2_provider) | **Post** /tenants/{tenant_id}/oauth2-providers | 
[**delete_tenant_oauth2_provider**](Oauth2ProviderApi.md#delete_tenant_oauth2_provider) | **Delete** /tenants/{tenant_id}/oauth2-providers/{tenant_oauht2_provider_id} | 
[**update_tenant_oauth2_provider**](Oauth2ProviderApi.md#update_tenant_oauth2_provider) | **Put** /tenants/{tenant_id}/oauth2-providers/{tenant_oauht2_provider_id} | 



## create_tenant_oauth2_provider

> models::TenantOAuth2Provider create_tenant_oauth2_provider(tenant_id, create_tenant_o_auth2_provider)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **i64** | Tenant ID | [required] |
**create_tenant_o_auth2_provider** | [**CreateTenantOAuth2Provider**](CreateTenantOAuth2Provider.md) |  | [required] |

### Return type

[**models::TenantOAuth2Provider**](TenantOAuth2Provider.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tenant_oauth2_provider

> delete_tenant_oauth2_provider(tenant_id, tenant_oauht2_provider_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **i64** | Tenant ID | [required] |
**tenant_oauht2_provider_id** | **i64** | OAuth2 Provider ID | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tenant_oauth2_provider

> update_tenant_oauth2_provider(tenant_id, tenant_oauht2_provider_id, update_tenant_o_auth2_provider)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **i64** | Tenant ID | [required] |
**tenant_oauht2_provider_id** | **i64** | OAuth2 Provider ID | [required] |
**update_tenant_o_auth2_provider** | [**UpdateTenantOAuth2Provider**](UpdateTenantOAuth2Provider.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

