# \UserApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_users**](UserApi.md#all_users) | **GET** /users | 
[**create_user**](UserApi.md#create_user) | **POST** /users | 
[**create_user_email**](UserApi.md#create_user_email) | **POST** /users/{user_id}/emails | 
[**create_user_phone_number**](UserApi.md#create_user_phone_number) | **POST** /users/{user_id}/phone_numbers | 
[**create_user_reset_password_token**](UserApi.md#create_user_reset_password_token) | **POST** /users/{user_id}/reset-password | 
[**delete_user**](UserApi.md#delete_user) | **DELETE** /users/{user_id} | 
[**delete_user_email**](UserApi.md#delete_user_email) | **DELETE** /users/{user_id}/emails/{email_id} | 
[**delete_user_phone_number**](UserApi.md#delete_user_phone_number) | **DELETE** /users/{user_id}/phone-numbers/{phone_number_id} | 
[**get_user_by_id**](UserApi.md#get_user_by_id) | **GET** /users/{user_id} | 
[**update_user**](UserApi.md#update_user) | **PUT** /users/{user_id} | 
[**update_user_email**](UserApi.md#update_user_email) | **PUT** /users/{user_id}/emails/{email_id} | 
[**update_user_info**](UserApi.md#update_user_info) | **PUT** /users/{user_id}/info | 
[**update_user_phone_number**](UserApi.md#update_user_phone_number) | **PUT** /users/{user_id}/phone-numbers/{phone_number_id} | 



## all_users

> models::UserPagination all_users(offset, limit, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**u32**> |  |  |
**limit** | Option<**u32**> |  |  |
**application_id** | Option<**i64**> |  |  |

### Return type

[**models::UserPagination**](UserPagination.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> models::User create_user(create_user, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user** | [**CreateUser**](CreateUser.md) |  | [required] |
**application_id** | Option<**i64**> |  |  |

### Return type

[**models::User**](User.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_email

> models::UserEmail create_user_email(user_id, service_account_create_user_email, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | User id | [required] |
**service_account_create_user_email** | [**ServiceAccountCreateUserEmail**](ServiceAccountCreateUserEmail.md) |  | [required] |
**application_id** | Option<**i64**> |  |  |

### Return type

[**models::UserEmail**](UserEmail.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_phone_number

> models::UserPhoneNumber create_user_phone_number(user_id, service_account_create_user_phone_number, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | User id | [required] |
**service_account_create_user_phone_number** | [**ServiceAccountCreateUserPhoneNumber**](ServiceAccountCreateUserPhoneNumber.md) |  | [required] |
**application_id** | Option<**i64**> |  |  |

### Return type

[**models::UserPhoneNumber**](UserPhoneNumber.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_reset_password_token

> models::Token create_user_reset_password_token(user_id, user_reset_password, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | User id | [required] |
**user_reset_password** | [**UserResetPassword**](UserResetPassword.md) |  | [required] |
**application_id** | Option<**i64**> |  |  |

### Return type

[**models::Token**](Token.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> delete_user(user_id, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | User id | [required] |
**application_id** | Option<**i64**> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_email

> delete_user_email(user_id, email_id, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | User id | [required] |
**email_id** | **i64** | Email id | [required] |
**application_id** | Option<**i64**> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_phone_number

> delete_user_phone_number(user_id, phone_number_id, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | User id | [required] |
**phone_number_id** | **i64** | PhoneNumber id | [required] |
**application_id** | Option<**i64**> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_id

> models::UserPagination get_user_by_id(user_id, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | User id | [required] |
**application_id** | Option<**i64**> |  |  |

### Return type

[**models::UserPagination**](UserPagination.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> update_user(user_id, update_user)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** |  | [required] |
**update_user** | [**UpdateUser**](UpdateUser.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_email

> update_user_email(user_id, email_id, service_account_update_user_email, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | User id | [required] |
**email_id** | **i64** | Email id | [required] |
**service_account_update_user_email** | [**ServiceAccountUpdateUserEmail**](ServiceAccountUpdateUserEmail.md) |  | [required] |
**application_id** | Option<**i64**> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_info

> update_user_info(user_id, update_user_info_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** |  | [required] |
**update_user_info_request** | [**UpdateUserInfoRequest**](UpdateUserInfoRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_phone_number

> update_user_phone_number(user_id, phone_number_id, service_account_update_user_phone_number, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | User id | [required] |
**phone_number_id** | **i64** | PhoneNumber id | [required] |
**service_account_update_user_phone_number** | [**ServiceAccountUpdateUserPhoneNumber**](ServiceAccountUpdateUserPhoneNumber.md) |  | [required] |
**application_id** | Option<**i64**> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

