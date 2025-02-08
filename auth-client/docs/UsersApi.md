# \UsersApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_users**](UsersApi.md#all_users) | **GET** /users | 
[**create_user**](UsersApi.md#create_user) | **POST** /users | 
[**create_user_reset_password_token**](UsersApi.md#create_user_reset_password_token) | **POST** /users/{user_id}/reset-password | 
[**get_user_by_id**](UsersApi.md#get_user_by_id) | **GET** /users/{user_id} | 



## all_users

> models::Pagination all_users(offset, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**u32**> |  |  |
**limit** | Option<**u32**> |  |  |

### Return type

[**models::Pagination**](Pagination.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> models::User create_user(create_user)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user** | [**CreateUser**](CreateUser.md) |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_reset_password_token

> models::Token create_user_reset_password_token(user_id, user_reset_password)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | User id | [required] |
**user_reset_password** | [**UserResetPassword**](UserResetPassword.md) |  | [required] |

### Return type

[**models::Token**](Token.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_id

> models::Pagination get_user_by_id(user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | User id | [required] |

### Return type

[**models::Pagination**](Pagination.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

