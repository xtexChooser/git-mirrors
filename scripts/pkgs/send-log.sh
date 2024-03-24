#!/usr/bin/env sh
set -e

command -v curl >/dev/null 2>&1 || apk add curl

[[ "${CI_WORKFLOW_NAME:-}" == "" ]] && echo "CI_WORKFLOW_NAME is not available" && exit 1
[[ "${CI_PIPELINE_STATUS:-}" == "" ]] && echo "CI_PIPELINE_STATUS is not available" && exit 1
[[ "${CI_PIPELINE_NUMBER:-}" == "" ]] && echo "CI_PIPELINE_NUMBER is not available" && exit 1
[[ "${CI_PIPELINE_EVENT:-}" == "" ]] && echo "CI_PIPELINE_EVENT is not available" && exit 1
[[ "${CI_STEP_URL:-}" == "" ]] && echo "CI_STEP_URL is not available" && exit 1
[[ "${NTFY_TOKEN:-}" == "" ]] && echo "NTFY_TOKEN is not available" && exit 1

pipeline=${CI_WORKFLOW_NAME#build-}
if [[ "$VERSION" != "" ]]; then
	pipeline="$pipeline $VERSION"
fi

desc="Pipeline: $CI_PIPELINE_NUMBER
Triggerred by: $CI_PIPELINE_EVENT"
priority=min

if [[ "$CI_PIPELINE_STATUS" == "success" ]]; then
	title="$pipeline build succeeded"
elif [[ "$CI_PIPELINE_STATUS" == "failure" ]]; then
	title="$pipeline build failed"
	priority=high
else
	title="pipelines: Pipeline status unknown ($CI_PIPELINE_STATUS): $pipeline"
	priority=high
fi

curl \
	-H "Authorization: Bearer $NTFY_TOKEN" \
	-H "X-Title: $title" \
	-H "X-Actions: view, View on CI, $CI_STEP_URL" \
	-H "X-Tags: pipline-$pipeline,pipeline-$CI_PIPELINE_STATUS" \
	-H "X-Priority: $priority" \
	-d "$desc" \
	-SL \
	https://ntfy.xvnet.eu.org/publogs
