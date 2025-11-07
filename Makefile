BIN_NAME := ft6336u-driver
INSTALL_DIR := /usr/local/bin

SERVICE_FILE := ft6336u-driver.service
SYSTEMD_DIR := /etc/systemd/system

all: build

build:
	I2C_DEV=$(I2C_DEV) UINPUT_DEV=$(UINPUT_DEV) cargo build --release

install:
	@echo "Installing driver to $(INSTALL_DIR)..."
	install -Dm755 target/release/$(BIN_NAME) $(INSTALL_DIR)/$(BIN_NAME)
	@echo "Coping systemd unit..."
	install -Dm644 $(SERVICE_FILE) $(SYSTEMD_DIR)/$(SERVICE_FILE)
	@echo "Reload systemd config..."
	systemctl daemon-reload
	@echo "Enable and launch service..."
	systemctl enable --now $(SERVICE_FILE)
	@echo "✅ Installing completed!"

uninstall:
	@echo "Stopping and disabling service..."
	-systemctl disable --now $(SERVICE_FILE)
	@echo "Removing unit..."
	rm -f $(SYSTEMD_DIR)/$(SERVICE_FILE)
	@echo "Removing driver..."
	rm -f $(INSTALL_DIR)/$(BIN_NAME)
	@echo "Reloading systemd config..."
	systemctl daemon-reload
	@echo "✅ Uninstalling complete!"

clean:
	cargo clean

.PHONY: all build install uninstall clean

