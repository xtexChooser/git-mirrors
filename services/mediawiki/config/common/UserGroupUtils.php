<?php

function xvRemoveGroup(string $group)
{
	/**
	 * @global array[] $wgGroupPermissions
	 * @global array[] $wgRevokePermissions
	 */
	unset($GLOBALS['wgGroupPermissions'][$group]);
	unset($GLOBALS['wgRevokePermissions'][$group]);
	foreach (['wgAddGroups', 'wgRemoveGroups', 'wgGroupsAddToSelf', 'wgGroupsRemoveFromSelf'] as $var) {
		unset($GLOBALS[$var][$group]);
		foreach ($GLOBALS[$var] as $key => &$val) {
			$val = array_diff($val, array($group));
		}
	}
}

function xvMergeGroup(string $from, string $to)
{
	foreach (['wgAddGroups', 'wgRemoveGroups', 'wgGroupsAddToSelf', 'wgGroupsRemoveFromSelf'] as $var) {
		if (!isset($GLOBALS[$var][$from]))
			continue;
		if (!isset($GLOBALS[$var][$to]))
			$GLOBALS[$var][$to] = [];
		$GLOBALS[$var][$to] += $GLOBALS[$var][$from];
	}
	foreach (['wgGroupPermissions', 'wgRevokePermissions'] as $var) {
		if (!isset($GLOBALS[$var][$from]))
			continue;
		if (!isset($GLOBALS[$var][$to]))
			$GLOBALS[$var][$to] = [];
		$GLOBALS[$var][$to] += $GLOBALS[$var][$from];
	}
	xvRemoveGroup($from);
}
