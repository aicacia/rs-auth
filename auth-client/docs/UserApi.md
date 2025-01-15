# \UserApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user_email**](UserApi.md#create_user_email) | **Post** /users/{user_id}/emails | 
[**create_user_phone_number**](UserApi.md#create_user_phone_number) | **Post** /users/{user_id}/phone_numbers | 
[**delete_user_email**](UserApi.md#delete_user_email) | **Delete** /users/{user_id}/emails/{email_id} | 
[**delete_user_phone_number**](UserApi.md#delete_user_phone_number) | **Delete** /users/{user_id}/phone-numbers/{phone_number_id} | 
[**update_user_email**](UserApi.md#update_user_email) | **Put** /users/{user_id}/emails/{email_id} | 
[**update_user_phone_number**](UserApi.md#update_user_phone_number) | **Put** /users/{user_id}/phone-numbers/{phone_number_id} | 



## create_user_email

> models::UserEmail create_user_email(user_id, service_account_create_user_email)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | User id | [required] |
**service_account_create_user_email** | [**ServiceAccountCreateUserEmail**](ServiceAccountCreateUserEmail.md) |  | [required] |

### Return type

[**models::UserEmail**](UserEmail.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_phone_number

> models::UserPhoneNumber create_user_phone_number(user_id, service_account_create_user_phone_number)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | User id | [required] |
**service_account_create_user_phone_number** | [**ServiceAccountCreateUserPhoneNumber**](ServiceAccountCreateUserPhoneNumber.md) |  | [required] |

### Return type

[**models::UserPhoneNumber**](UserPhoneNumber.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_email

> delete_user_email(user_id, email_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | User id | [required] |
**email_id** | **i64** | Email id | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_phone_number

> delete_user_phone_number(user_id, phone_number_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | User id | [required] |
**phone_number_id** | **i64** | PhoneNumber id | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_email

> update_user_email(user_id, email_id, service_account_update_user_email)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | User id | [required] |
**email_id** | **i64** | Email id | [required] |
**service_account_update_user_email** | [**ServiceAccountUpdateUserEmail**](ServiceAccountUpdateUserEmail.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_phone_number

> update_user_phone_number(user_id, phone_number_id, service_account_update_user_phone_number)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | User id | [required] |
**phone_number_id** | **i64** | PhoneNumber id | [required] |
**service_account_update_user_phone_number** | [**ServiceAccountUpdateUserPhoneNumber**](ServiceAccountUpdateUserPhoneNumber.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

