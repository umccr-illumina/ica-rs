# Rust API client for openapi

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v1
- Package version: v1
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://aps2.platform.illumina.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*TaskRunsApi* | [**abort_task_run**](docs/TaskRunsApi.md#abort_task_run) | **PUT** /v1/tasks/runs/{runId}:abort | Abort a task run
*TaskRunsApi* | [**create_task_run**](docs/TaskRunsApi.md#create_task_run) | **POST** /v1/tasks/runs | Create and launch a task run
*TaskRunsApi* | [**get_task_run**](docs/TaskRunsApi.md#get_task_run) | **GET** /v1/tasks/runs/{runId} | Get the status of a task run
*TaskRunsApi* | [**heartbeat_task_run**](docs/TaskRunsApi.md#heartbeat_task_run) | **POST** /v1/tasks/runs/{runId}:heartbeat | Heartbeat for a task run
*TaskRunsApi* | [**list_task_runs**](docs/TaskRunsApi.md#list_task_runs) | **GET** /v1/tasks/runs | Get a list of task runs
*TaskVersionsApi* | [**create_task_version**](docs/TaskVersionsApi.md#create_task_version) | **POST** /v1/tasks/{taskId}/versions | Create a task version
*TaskVersionsApi* | [**get_task_version**](docs/TaskVersionsApi.md#get_task_version) | **GET** /v1/tasks/{taskId}/versions/{versionId} | Get the details of a task version
*TaskVersionsApi* | [**launch_task_run**](docs/TaskVersionsApi.md#launch_task_run) | **POST** /v1/tasks/{taskId}/versions/{versionId}:launch | Launch a task version
*TaskVersionsApi* | [**list_task_versions**](docs/TaskVersionsApi.md#list_task_versions) | **GET** /v1/tasks/{taskId}/versions | Get a list of versions
*TaskVersionsApi* | [**update_task_version**](docs/TaskVersionsApi.md#update_task_version) | **PATCH** /v1/tasks/{taskId}/versions/{versionId} | Update task version properties
*TasksApi* | [**create_task**](docs/TasksApi.md#create_task) | **POST** /v1/tasks | Create a Task
*TasksApi* | [**get_task**](docs/TasksApi.md#get_task) | **GET** /v1/tasks/{taskId} | Get the details of a Task
*TasksApi* | [**list_tasks**](docs/TasksApi.md#list_tasks) | **GET** /v1/tasks | Get a list of tasks
*TasksApi* | [**update_task**](docs/TasksApi.md#update_task) | **PATCH** /v1/tasks/{taskId} | Update an existing task.


## Documentation For Models

 - [ContainerState](docs/ContainerState.md)
 - [ContainerStateRunning](docs/ContainerStateRunning.md)
 - [ContainerStateTerminated](docs/ContainerStateTerminated.md)
 - [ContainerStateWaiting](docs/ContainerStateWaiting.md)
 - [ContainerStatus](docs/ContainerStatus.md)
 - [CreateTaskRequest](docs/CreateTaskRequest.md)
 - [CreateTaskRunRequest](docs/CreateTaskRunRequest.md)
 - [CreateTaskVersionRequest](docs/CreateTaskVersionRequest.md)
 - [Credentials](docs/Credentials.md)
 - [Environment](docs/Environment.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [Execution](docs/Execution.md)
 - [HeartbeatTaskRunRequest](docs/HeartbeatTaskRunRequest.md)
 - [Image](docs/Image.md)
 - [InputMountMappingWithCreds](docs/InputMountMappingWithCreds.md)
 - [InputStreamSettings](docs/InputStreamSettings.md)
 - [LaunchTaskRequest](docs/LaunchTaskRequest.md)
 - [MountMappingWithCreds](docs/MountMappingWithCreds.md)
 - [Resources](docs/Resources.md)
 - [SystemFiles](docs/SystemFiles.md)
 - [Task](docs/Task.md)
 - [TaskRun](docs/TaskRun.md)
 - [TaskRunHeartbeat](docs/TaskRunHeartbeat.md)
 - [TaskRunLogs](docs/TaskRunLogs.md)
 - [TaskRunSummary](docs/TaskRunSummary.md)
 - [TaskRunSummaryPagedItems](docs/TaskRunSummaryPagedItems.md)
 - [TaskSummary](docs/TaskSummary.md)
 - [TaskSummaryPagedItems](docs/TaskSummaryPagedItems.md)
 - [TaskVersion](docs/TaskVersion.md)
 - [TaskVersionSummary](docs/TaskVersionSummary.md)
 - [TaskVersionSummaryPagedItems](docs/TaskVersionSummaryPagedItems.md)
 - [UpdateTaskRequest](docs/UpdateTaskRequest.md)
 - [UpdateTaskVersionRequest](docs/UpdateTaskVersionRequest.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



