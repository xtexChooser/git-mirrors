// Copyright 2024 The Forgejo Authors. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later

package repo

import (
	"net/http"

	issues_model "code.gitea.io/gitea/models/issues"
	access_model "code.gitea.io/gitea/models/perm/access"
	"code.gitea.io/gitea/modules/setting"
	"code.gitea.io/gitea/services/context"
)

// AddParentIssue adds a parent issue
func AddParentIssue(ctx *context.Context) {
	issueIndex := ctx.ParamsInt64("index")
	parentID := ctx.FormInt64("parentIssue")

	issue, err := issues_model.GetIssueByIndex(ctx, ctx.Repo.Repository.ID, issueIndex)
	if err != nil {
		ctx.ServerError("GetIssueByIndex", err)
		return
	}

	if !ctx.Repo.CanUpdateParentIssues(ctx, ctx.Doer) {
		ctx.Error(http.StatusForbidden, "CanUpdateParentIssues")
		return
	}

	if err = issue.LoadRepo(ctx); err != nil {
		ctx.ServerError("LoadRepo", err)
		return
	}

	// Redirect
	defer ctx.Redirect(issue.Link())

	parent, err := issues_model.GetIssueByID(ctx, parentID)
	if err != nil {
		ctx.Flash.Error(ctx.Tr("repo.issues.parent_issue.add_error_issue_not_exist"))
		return
	}

	// Check if both issues are in the same repo if cross repository dependencies is not enabled
	if issue.RepoID != parent.RepoID {
		if !setting.Service.AllowCrossRepositoryDependencies {
			ctx.Flash.Error(ctx.Tr("repo.issues.parent_issue.add_error_not_same_repo"))
			return
		}
		if err := parent.LoadRepo(ctx); err != nil {
			ctx.ServerError("loadRepo", err)
			return
		}
		// Can ctx.Doer write issues in the target repo?
		perm, err := access_model.GetUserRepoPermission(ctx, parent.Repo, ctx.Doer)
		if err != nil {
			ctx.ServerError("GetUserRepoPermission", err)
			return
		}
		if !perm.CanWriteIssuesOrPulls(false) {
			ctx.Flash.Error(ctx.Tr("repo.issues.parent_issue.add_error_cannot_write_repo"))
			return
		}
	}

	if err = issue.UpdateParentIssue(ctx, parent, ctx.Doer); err != nil {
		if issues_model.IsErrSubIssuesTooMany(err) {
			ctx.Flash.Error(ctx.Tr("repo.issues.parent_issue.add_error_count_limit"))
			return
		} else if issues_model.IsErrSubIssuesTooDeep(err) {
			ctx.Flash.Error(ctx.Tr("repo.issues.parent_issue.add_error_depth_limit"))
			return
		} else if issues_model.IsErrCircularParentIssue(err) {
			ctx.Flash.Error(ctx.Tr("repo.issues.parent_issue.add_error_circular"))
			return
		}
		ctx.ServerError("UpdateParentIssue, add", err)
		return
	}
}

// RemoveParentIssue removes parent issue from a issue
func RemoveParentIssue(ctx *context.Context) {
	issueIndex := ctx.ParamsInt64("index")

	issue, err := issues_model.GetIssueByIndex(ctx, ctx.Repo.Repository.ID, issueIndex)
	if err != nil {
		ctx.ServerError("GetIssueByIndex", err)
		return
	}

	if !ctx.Repo.CanUpdateParentIssues(ctx, ctx.Doer) {
		ctx.Error(http.StatusForbidden, "CanUpdateParentIssues")
		return
	}

	if err = issue.LoadRepo(ctx); err != nil {
		ctx.ServerError("LoadRepo", err)
		return
	}

	// Redirect
	defer ctx.Redirect(issue.Link())

	if err = issue.UpdateParentIssue(ctx, nil, ctx.Doer); err != nil {
		ctx.ServerError("UpdateParentIssue, remove", err)
		return
	}
}
