<?php

/**
 * Grant a permission to many groups
 * 
 * @param string $permission Right
 * @param string[] $groups Groups
 * @return void
 */
function xvGrantPermission(string $permission, array $groups)
{
	/**
	 * @global array[] $wgGroupPermissions
	 */
	global $wgGroupPermissions;
	foreach ($groups as $group)
		$wgGroupPermissions[$group][$permission] = true;
}

/**
 * Grant many permissions to a group
 * @param string $group Group
 * @param string[] $permissions Rights
 * @return void
 */
function xvGrantPermissionsTo(string $group, array $permissions)
{
	/**
	 * @global array[] $wgGroupPermissions
	 */
	global $wgGroupPermissions;
	foreach ($permissions as $permission)
		$wgGroupPermissions[$group][$permission] = true;
}

/**
 * Remove a permission from many groups
 * @param string $permission Right
 * @param string[] $groups Groups
 * @return void
 */
function xvRemovePermission(string $permission, array $groups)
{
	/**
	 * @global array[] $wgGroupPermissions
	 */
	global $wgGroupPermissions;
	foreach ($groups as $group)
		$wgGroupPermissions[$group][$permission] = false;
}

/**
 * Remove many permissions from a group
 * @param string $group Group
 * @param string[] $permissions Rights
 * @return void
 */
function xvRemovePermissionsFrom(string $group, array $permissions)
{
	/**
	 * @global array[] $wgGroupPermissions
	 */
	global $wgGroupPermissions;
	foreach ($permissions as $permission)
		$wgGroupPermissions[$group][$permission] = false;
}

/**
 * Set variable[keys] to true
 * @param string $variable Options variable
 * @param array $keys Keys to be set
 * @return void
 */
function xvSetAssocTrues(string $variable, array $keys)
{
	foreach ($keys as $key)
		$GLOBALS[$variable][$key] = true;
}

/**
 * Merge array into options
 * 
 * @param string $variable Options variable
 * @param array[] $values Arrays to be merge into
 * @return void
 */
function xvMergeInto(string $variable, array ...$values)
{
	$GLOBALS[$variable] = array_merge($GLOBALS[$variable], ...$values);
}

/**
 * Merge array into options
 * 
 * @param string $variable Options variable
 * @param string $k
 * @param array[] $values Arrays to be merge into
 * @return void
 */
function xvMergeInto2(string $variable, string $k, array ...$values)
{
	$GLOBALS[$variable][$k] = array_merge($GLOBALS[$variable][$k], ...$values);
}

/**
 * Check if an extension is loaded
 * 
 * @param string $extension Extension name
 * @return bool Returns true if the given extension is loaded
 */
function xvIsExtensionLoaded(string $extension)
{
	return ExtensionRegistry::getInstance()->isLoaded($extension);
}

/**
 * Load json from file
 * @param string $file
 * @return array
 */
function xvLoadJson(string $file): array
{
	return json_decode(file_get_contents("/etc/mediawiki/$file"), true);
}
