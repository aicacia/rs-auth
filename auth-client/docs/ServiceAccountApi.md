# \ServiceAccountApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_service_accounts**](ServiceAccountApi.md#all_service_accounts) | **GET** /service-accounts | 
[**create_service_account**](ServiceAccountApi.md#create_service_account) | **POST** /service-accounts | 
[**delete_service_account**](ServiceAccountApi.md#delete_service_account) | **DELETE** /service-accounts/{service_account_id} | 
[**get_service_account_by_id**](ServiceAccountApi.md#get_service_account_by_id) | **GET** /service-accounts/{service_account_id} | 
[**update_service_account**](ServiceAccountApi.md#update_service_account) | **PUT** /service-accounts/{service_account_id} | 



## all_service_accounts

> models::ServiceAccountPagination all_service_accounts(offset, limit, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**u32**> |  |  |
**limit** | Option<**u32**> |  |  |
**application_id** | Option<**i64**> |  |  |

### Return type

[**models::ServiceAccountPagination**](ServiceAccountPagination.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_service_account

> models::ServiceAccount create_service_account(create_service_account, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_service_account** | [**CreateServiceAccount**](CreateServiceAccount.md) |  | [required] |
**application_id** | Option<**i64**> |  |  |

### Return type

[**models::ServiceAccount**](ServiceAccount.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_service_account

> delete_service_account(service_account_id, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account_id** | **i64** | ServiceAccount ID | [required] |
**application_id** | Option<**i64**> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_account_by_id

> models::ServiceAccount get_service_account_by_id(service_account_id, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account_id** | **i64** | ServiceAccount ID | [required] |
**application_id** | Option<**i64**> |  |  |

### Return type

[**models::ServiceAccount**](ServiceAccount.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_service_account

> models::ServiceAccount update_service_account(service_account_id, update_service_account, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account_id** | **i64** | ServiceAccount ID | [required] |
**update_service_account** | [**UpdateServiceAccount**](UpdateServiceAccount.md) |  | [required] |
**application_id** | Option<**i64**> |  |  |

### Return type

[**models::ServiceAccount**](ServiceAccount.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

