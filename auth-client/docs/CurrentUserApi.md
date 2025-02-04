# \CurrentUserApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_current_user_add_oauth2_provider_url**](CurrentUserApi.md#create_current_user_add_oauth2_provider_url) | **POST** /current-user/oauth2/{provider} | 
[**create_current_user_email**](CurrentUserApi.md#create_current_user_email) | **POST** /current-user/emails | 
[**create_current_user_phone_number**](CurrentUserApi.md#create_current_user_phone_number) | **POST** /current-user/phone-numbers | 
[**create_current_user_totp**](CurrentUserApi.md#create_current_user_totp) | **POST** /current-user/totp | 
[**deactivate_current_user**](CurrentUserApi.md#deactivate_current_user) | **DELETE** /current-user | 
[**delete_current_user_email**](CurrentUserApi.md#delete_current_user_email) | **DELETE** /current-user/emails/{email_id} | 
[**delete_current_user_phone_number**](CurrentUserApi.md#delete_current_user_phone_number) | **DELETE** /current-user/phone-numbers/{phone_number_id} | 
[**delete_current_user_totp**](CurrentUserApi.md#delete_current_user_totp) | **DELETE** /current-user/totp | 
[**get_current_user**](CurrentUserApi.md#get_current_user) | **GET** /current-user | 
[**reset_current_user_password**](CurrentUserApi.md#reset_current_user_password) | **POST** /current-user/reset-password | 
[**set_current_user_email_as_primary**](CurrentUserApi.md#set_current_user_email_as_primary) | **PUT** /current-user/emails/{email_id}/set-as-primary | 
[**set_current_user_phone_number_as_primary**](CurrentUserApi.md#set_current_user_phone_number_as_primary) | **PUT** /current-user/phone-numbers/{phone_number_id}/set-as-primary | 
[**update_current_user**](CurrentUserApi.md#update_current_user) | **PUT** /current-user | 
[**update_current_user_config**](CurrentUserApi.md#update_current_user_config) | **PUT** /current-user/config | 
[**update_current_user_info**](CurrentUserApi.md#update_current_user_info) | **PUT** /current-user/info | 



## create_current_user_add_oauth2_provider_url

> String create_current_user_add_oauth2_provider_url(provider, state)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | **String** | OAuth2 provider | [required] |
**state** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_current_user_email

> models::UserEmail create_current_user_email(create_user_email)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_email** | [**CreateUserEmail**](CreateUserEmail.md) |  | [required] |

### Return type

[**models::UserEmail**](UserEmail.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_current_user_phone_number

> models::UserPhoneNumber create_current_user_phone_number(create_user_phone_number)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_phone_number** | [**CreateUserPhoneNumber**](CreateUserPhoneNumber.md) |  | [required] |

### Return type

[**models::UserPhoneNumber**](UserPhoneNumber.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_current_user_totp

> models::UserTotp create_current_user_totp(create_totp_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_totp_request** | [**CreateTotpRequest**](CreateTotpRequest.md) |  | [required] |

### Return type

[**models::UserTotp**](UserTOTP.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deactivate_current_user

> deactivate_current_user()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_current_user_email

> delete_current_user_email(email_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | **i64** | Email ID to delete | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_current_user_phone_number

> delete_current_user_phone_number(phone_number_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_number_id** | **i64** | PhoneNumber ID to delete | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_current_user_totp

> delete_current_user_totp()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user

> models::User get_current_user()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::User**](User.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_current_user_password

> reset_current_user_password(reset_password_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reset_password_request** | [**ResetPasswordRequest**](ResetPasswordRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_current_user_email_as_primary

> set_current_user_email_as_primary(email_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | **i64** | Email ID to set as primary | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_current_user_phone_number_as_primary

> set_current_user_phone_number_as_primary(phone_number_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_number_id** | **i64** | PhoneNumber ID to set as primary | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_current_user

> update_current_user(update_username)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_username** | [**UpdateUsername**](UpdateUsername.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_current_user_config

> update_current_user_config(update_user_config_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_user_config_request** | [**UpdateUserConfigRequest**](UpdateUserConfigRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_current_user_info

> update_current_user_info(update_user_info_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_user_info_request** | [**UpdateUserInfoRequest**](UpdateUserInfoRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

