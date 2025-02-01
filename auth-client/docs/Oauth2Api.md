# \Oauth2Api

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_oauth2_url**](Oauth2Api.md#create_oauth2_url) | **POST** /oauth2/{provider} | 
[**oauth2_callback**](Oauth2Api.md#oauth2_callback) | **GET** /oauth2/{provider}/callback | 



## create_oauth2_url

> String create_oauth2_url(provider, register)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | **String** | OAuth2 provider | [required] |
**register** | Option<**bool**> |  |  |

### Return type

**String**

### Authorization

[TenantUUID](../README.md#TenantUUID)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_callback

> oauth2_callback(provider, state, code, scope)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | **String** | OAuth2 provider | [required] |
**state** | **String** |  | [required] |
**code** | **String** |  | [required] |
**scope** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

