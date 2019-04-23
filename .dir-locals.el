((nil . ((projectile-project-compilation-cmd . "cmake --build build --config Release --target p2studio")
		 (projectile-project-run-cmd . "./build_and_run.sh")
		 (counsel-etags-update-tags-backend . (lambda (src-dir) (shell-command "rusty-tags emacs"))))))
