# \TenantApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_tenants**](TenantApi.md#all_tenants) | **Get** /tenants | 
[**create_tenant**](TenantApi.md#create_tenant) | **Post** /tenants | 
[**delete_tenant**](TenantApi.md#delete_tenant) | **Delete** /tenants/{tenant_id} | 
[**get_tenant_by_client_id**](TenantApi.md#get_tenant_by_client_id) | **Get** /tenants/by-client-id/{tenant_client_id} | 
[**get_tenant_by_id**](TenantApi.md#get_tenant_by_id) | **Get** /tenants/{tenant_id} | 
[**update_tenant**](TenantApi.md#update_tenant) | **Put** /tenants/{tenant_id} | 



## all_tenants

> models::PaginationTenant all_tenants(offset, limit, show_private_key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> |  |  |
**limit** | Option<**i32**> |  |  |
**show_private_key** | Option<**bool**> |  |  |

### Return type

[**models::PaginationTenant**](Pagination_Tenant.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_tenant

> models::Tenant create_tenant(create_tenant)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_tenant** | [**CreateTenant**](CreateTenant.md) |  | [required] |

### Return type

[**models::Tenant**](Tenant.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tenant

> delete_tenant(tenant_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **i64** | Tenant ID | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant_by_client_id

> models::Tenant get_tenant_by_client_id(tenant_client_id, show_private_key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_client_id** | **uuid::Uuid** | Tenant ID | [required] |
**show_private_key** | Option<**bool**> |  |  |

### Return type

[**models::Tenant**](Tenant.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant_by_id

> models::Tenant get_tenant_by_id(tenant_id, show_private_key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **i64** | Tenant ID | [required] |
**show_private_key** | Option<**bool**> |  |  |

### Return type

[**models::Tenant**](Tenant.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tenant

> models::Tenant update_tenant(tenant_id, update_tenant)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **i64** | Tenant ID | [required] |
**update_tenant** | [**UpdateTenant**](UpdateTenant.md) |  | [required] |

### Return type

[**models::Tenant**](Tenant.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

