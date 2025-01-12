// Copyright 2021 The Gitea Authors. All rights reserved.
// SPDX-License-Identifier: MIT

package context

import (
	"fmt"
	"net/http"

	actions_model "code.gitea.io/gitea/models/actions"
	"code.gitea.io/gitea/models/organization"
	packages_model "code.gitea.io/gitea/models/packages"
	"code.gitea.io/gitea/models/perm"
	"code.gitea.io/gitea/models/unit"
	user_model "code.gitea.io/gitea/models/user"
	actions_module "code.gitea.io/gitea/modules/actions"
	"code.gitea.io/gitea/modules/setting"
	"code.gitea.io/gitea/modules/structs"
	"code.gitea.io/gitea/modules/templates"
)

// Package contains owner, access mode and optional the package descriptor
type Package struct {
	Owner      *user_model.User
	AccessMode perm.AccessMode
	Descriptor *packages_model.PackageDescriptor
}

type packageAssignmentCtx struct {
	*Base
	Doer        *user_model.User
	ContextUser *user_model.User
}

// PackageAssignment returns a middleware to handle Context.Package assignment
func PackageAssignment(packageType packages_model.Type) func(ctx *Context) {
	return func(ctx *Context) {
		errorFn := func(status int, title string, obj any) {
			err, ok := obj.(error)
			if !ok {
				err = fmt.Errorf("%s", obj)
			}
			if status == http.StatusNotFound {
				ctx.NotFound(title, err)
			} else {
				ctx.ServerError(title, err)
			}
		}
		paCtx := &packageAssignmentCtx{Base: ctx.Base, Doer: ctx.Doer, ContextUser: ctx.ContextUser}
		ctx.Package = packageAssignment(paCtx, packageType, errorFn)
	}
}

// PackageAssignmentAPI returns a middleware to handle Context.Package assignment
func PackageAssignmentAPI(packageType packages_model.Type) func(ctx *APIContext) {
	return func(ctx *APIContext) {
		paCtx := &packageAssignmentCtx{Base: ctx.Base, Doer: ctx.Doer, ContextUser: ctx.ContextUser}
		ctx.Package = packageAssignment(paCtx, packageType, ctx.Error)
	}
}

func packageAssignment(ctx *packageAssignmentCtx, packageType packages_model.Type, errCb func(int, string, any)) *Package {
	pkg := &Package{
		Owner: ctx.ContextUser,
	}
	var err error

	isForgejoMgmtAPI := packageType == ""
	if isForgejoMgmtAPI {
		// for Forgejo packages management APIs, extract type from params
		packageType = packages_model.Type(ctx.Params("type"))
	}
	name := ctx.Params("name")
	if name == "" {
		name = ctx.Params("packagename")
	}
	version := ctx.Params("version")
	if version == "" {
		version = ctx.Params("packageversion")
	}
	if packageType != "" && name != "" && version != "" {
		pv, err := packages_model.GetVersionByNameAndVersion(ctx, pkg.Owner.ID, packageType, name, version)
		if err != nil {
			if err == packages_model.ErrPackageNotExist {
				if isForgejoMgmtAPI {
					errCb(http.StatusNotFound, "GetVersionByNameAndVersion", err)
					return pkg
				}
			} else {
				errCb(http.StatusInternalServerError, "GetVersionByNameAndVersion", err)
				return pkg
			}
		} else {
			pkg.Descriptor, err = packages_model.GetPackageDescriptor(ctx, pv)
			if err != nil {
				errCb(http.StatusInternalServerError, "GetPackageDescriptor", err)
				return pkg
			}
		}
	}

	pkg.AccessMode, err = determineAccessMode(ctx.Base, pkg, ctx.Doer)
	if err != nil {
		errCb(http.StatusInternalServerError, "determineAccessMode", err)
		return pkg
	}

	return pkg
}

func determineAccessMode(ctx *Base, pkg *Package, doer *user_model.User) (perm.AccessMode, error) {
	if setting.Service.RequireSignInView && (doer == nil || doer.IsGhost()) {
		return perm.AccessModeNone, nil
	}

	if doer != nil && !doer.IsGhost() && (!doer.IsActive || doer.ProhibitLogin) {
		return perm.AccessModeNone, nil
	}

	accessMode := perm.AccessModeNone

	if ctx.Data["IsActionsToken"] == true {
		taskID := ctx.Data["ActionsTaskID"].(int64)
		task, err := actions_model.GetTaskByID(ctx, taskID)
		if err != nil {
			return perm.AccessModeNone, err
		}

		// if the package is linked to the repository, grant write access
		// or else fall through to normal checks to avoid Actions accessing
		// private packages accidentally
		if pkg.Descriptor != nil && task.Status == actions_model.StatusRunning {
			repo := pkg.Descriptor.Repository
			if err = task.LoadJob(ctx); err != nil {
				return perm.AccessModeNone, err
			}
			if err = task.Job.LoadRun(ctx); err != nil {
				return perm.AccessModeNone, err
			}
			if repo != nil && task.RepoID == repo.ID &&
				task.Job.Run.TriggerEvent != actions_module.GithubEventPullRequest &&
				task.Job.Run.TriggerEvent != actions_module.GithubEventPullRequestTarget {
				return perm.AccessModeWrite, err
			}
		}
	}

	if pkg.Owner.IsOrganization() {
		org := organization.OrgFromUser(pkg.Owner)

		if doer != nil && !doer.IsGhost() {
			// 1. If user is logged in, check all team packages permissions
			var err error
			accessMode, err = org.GetOrgUserMaxAuthorizeLevel(ctx, doer.ID)
			if err != nil {
				return accessMode, err
			}
			// If access mode is less than write check every team for more permissions
			// The minimum possible access mode is read for org members
			if accessMode < perm.AccessModeWrite {
				teams, err := organization.GetUserOrgTeams(ctx, org.ID, doer.ID)
				if err != nil {
					return accessMode, err
				}
				for _, t := range teams {
					perm := t.UnitAccessMode(ctx, unit.TypePackages)
					if accessMode < perm {
						accessMode = perm
					}
				}
			}
		}
		if accessMode == perm.AccessModeNone && organization.HasOrgOrUserVisible(ctx, pkg.Owner, doer) {
			// 2. If user is unauthorized or no org member, check if org is visible
			accessMode = perm.AccessModeRead
		}
	} else {
		if doer != nil && !doer.IsGhost() {
			// 1. Check if user is package owner
			if doer.ID == pkg.Owner.ID {
				accessMode = perm.AccessModeOwner
			} else if pkg.Owner.Visibility == structs.VisibleTypePublic || pkg.Owner.Visibility == structs.VisibleTypeLimited { // 2. Check if package owner is public or limited
				accessMode = perm.AccessModeRead
			}
		} else if pkg.Owner.Visibility == structs.VisibleTypePublic { // 3. Check if package owner is public
			accessMode = perm.AccessModeRead
		}
	}

	return accessMode, nil
}

// PackageContexter initializes a package context for a request.
func PackageContexter() func(next http.Handler) http.Handler {
	renderer := templates.HTMLRenderer()
	return func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(resp http.ResponseWriter, req *http.Request) {
			base, baseCleanUp := NewBaseContext(resp, req)
			defer baseCleanUp()

			// it is still needed when rendering 500 page in a package handler
			ctx := NewWebContext(base, renderer, nil)
			ctx.Base.AppendContextValue(WebContextKey, ctx)
			next.ServeHTTP(ctx.Resp, ctx.Req)
		})
	}
}
