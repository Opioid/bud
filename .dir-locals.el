((nil . ((projectile-project-compilation-cmd . "cargo check")
		 (projectile-project-run-cmd . "./build_and_run.sh")
		 (counsel-etags-update-tags-backend . (lambda (src-dir) (shell-command "rusty-tags emacs"))))))
