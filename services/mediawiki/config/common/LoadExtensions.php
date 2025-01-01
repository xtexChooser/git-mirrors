<?php
// global extensions
if ($xvUseGlobalExtensions)
	xvLoadExtensions($xvGlobalExtensions);

// global skins
if ($xvUseGlobalSkins)
	xvLoadSkins($xvGlobalSkins);

if ($xvUseCaptcha)
	xvLoadExtension('ConfirmEdit');

if ($xvUseVisualEditor)
	xvLoadExtension('VisualEditor');

if ($xvUseMobileFrontend)
	xvLoadExtension('MobileFrontend');

if ($xvUseCaptcha) {
	switch ($xvCaptchaType) {
		case XvCaptchaType::Turnstile:
			xvLoadExtension('ConfirmEdit/Turnstile');
			$wgTurnstileSendRemoteIP = false;
			break;
		case XvCaptchaType::Questy:
			xvLoadExtension('ConfirmEdit/QuestyCaptcha');
			break;
	}
}

if ($xvUseLockdown)
	xvLoadExtension('Lockdown');

if ($xvUseCargo)
	xvLoadExtension('Cargo');

if ($xvUseContactPage)
	xvLoadExtension('ContactPage');

if ($xvUseSecurePoll)
	xvLoadExtension('SecurePoll');

if ($xvUseTabberNeue)
	xvLoadExtension('TabberNeue');
