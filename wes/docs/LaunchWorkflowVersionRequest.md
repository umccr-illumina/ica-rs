# LaunchWorkflowVersionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the workflow run | [optional]
**input** | Option<[**serde_json::Value**](.md)> | Input for the launched workflow run. Must resolve to a JSON object. | [optional]
**engine_parameters** | Option<[**serde_json::Value**](.md)> | Runtime options for launching workflows (currently only used for Airflow     and otherwise ignored). Must resolve to a JSON object. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


