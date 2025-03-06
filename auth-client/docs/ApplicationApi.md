# \ApplicationApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_applications**](ApplicationApi.md#all_applications) | **GET** /applications | 
[**create_application**](ApplicationApi.md#create_application) | **POST** /applications | 
[**delete_application**](ApplicationApi.md#delete_application) | **DELETE** /applications/{application_id} | 
[**get_application_by_id**](ApplicationApi.md#get_application_by_id) | **GET** /applications/{application_id} | 
[**update_application**](ApplicationApi.md#update_application) | **PUT** /applications/{application_id} | 



## all_applications

> models::ApplicationPagination all_applications(offset, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**u32**> |  |  |
**limit** | Option<**u32**> |  |  |

### Return type

[**models::ApplicationPagination**](ApplicationPagination.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_application

> models::Application create_application(create_application)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_application** | [**CreateApplication**](CreateApplication.md) |  | [required] |

### Return type

[**models::Application**](Application.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_application

> delete_application(application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **i64** | Application ID | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_application_by_id

> models::Application get_application_by_id(application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **i64** | Application ID | [required] |

### Return type

[**models::Application**](Application.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application

> models::Application update_application(application_id, update_application)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **i64** | Application ID | [required] |
**update_application** | [**UpdateApplication**](UpdateApplication.md) |  | [required] |

### Return type

[**models::Application**](Application.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

