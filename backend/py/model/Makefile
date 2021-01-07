build-MotokoModel:
	cp *.py $(ARTIFACTS_DIR)
	cp requirements.txt $(ARTIFACTS_DIR)
	pip install -r requirements.txt -t $(ARTIFACTS_DIR)
	rm -rf $(ARTIFACTS_DIR)/bin
	find $(ARTIFACTS_DIR) -name 'tests' -type d -prune -exec rm -rf '{}' +
	find $(ARTIFACTS_DIR) -name 'distutils' -type d -prune -exec rm -rf '{}' +
	find $(ARTIFACTS_DIR) -name 'datasets' -type d -prune -exec rm -rf '{}' +
	find $(ARTIFACTS_DIR) -name '_examples' -type d -prune -exec rm -rf '{}' +
	find $(ARTIFACTS_DIR) -name '__pycache__' -type d -prune -exec rm -rf '{}' +
