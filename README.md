# Grafana resource

A Concourse resource to update Grafana dashboard panels.

## Source Configuration

| Parameter          | Required      | Example                  | Description
| ------------------ | ------------- | -------------            | ------------------------------ |
| grafana_url        | Yes           | https://YOUR-GRAFANA.COM |  you grafana domain url                              |
| grafana_token      | Yes           | {YOUR_GRAFANA_TOKEN}     |                                |


### `out`: Update or Create Grafana dashboard panels.

Given a panels specified by `panels`, to update/create Grafana dashboard panels.

#### Parameters

* `dashboard_id`: *Required.* grafana dashboard ID.

* `panels`: *Required.* .json panels String provided by an output of a task.
.

## Example Configuration

### Resource type

``` yaml
resource_types:
- name: grafana-resource
    type: docker-image
    source:
      repository: teliaoss/grafana-resource
```

### Resource

``` yaml
resource:
- name: grafana-resource
    type: resource
    source:
      grafana_url: ((grafana-url))
      grafana_token: ((grafana-token))
```

### Plan

``` yaml
- put: grafana-resource
  params: 
    dashboard_id: ((dashboard-id))
    panels: "path/to/panels.json"
```

### Panels file example

[panels.json] (https://github.com/telia-oss/appsync-resource/blob/master/panels.json)


