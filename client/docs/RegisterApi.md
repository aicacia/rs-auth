# \RegisterApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**register_user**](RegisterApi.md#register_user) | **Post** /register | 



## register_user

> models::User register_user(register_user)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_user** | [**RegisterUser**](RegisterUser.md) |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[TenantUUID](../README.md#TenantUUID)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

