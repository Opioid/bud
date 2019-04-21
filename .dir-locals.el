((nil . ((projectile-project-compilation-cmd . "cmake --build build --config Release --target p2studio")
		 (projectile-project-run-cmd . "cd system && cargo run -- -i \"takes/model_test.take\" -t -1")
		 (counsel-etags-update-tags-backend . (lambda (src-dir) (shell-command "rusty-tags emacs"))))))
