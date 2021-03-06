# ProjectPagedItems

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**items** | Option<[**Vec<crate::models::Project>**](Project.md)> | Items in paged list | [optional]
**item_count** | Option<**i32**> | Number of items included in the page | [optional]
**first_page_token** | Option<**String**> | PageToken for first paged list | [optional]
**next_page_token** | Option<**String**> | PageToken for the next paged list | [optional]
**prev_page_token** | Option<**String**> | PageToken for the previous paged list | [optional]
**last_page_token** | Option<**String**> | PageToken for the last paged list. Only included when totalItemCount is listed | [optional]
**total_item_count** | Option<**i64**> | Total number of items in all pages. Only included when requested | [optional]
**total_page_count** | Option<**i64**> | Total number of pages. Only included when totalItemCount is listed | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


