#!/bin/bash
git push origin main

ssh salt.infra.xvnet.eu.org "sudo salt-run --force-color state.orch orch.deploy; sudo salt --force-color '*' state.apply"
