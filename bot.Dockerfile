#
# RustBot Docker Image
#
# This image is reponsible for running the bot itself
#
# We are using centos due to podman's relationship with red hat
FROM quay.io/containers/podman:latest

WORKDIR /home/podman

# This assumes the release build is done and the RPM package is built
COPY target/generate-rpm/rustbot-x86_64.rpm /home/podman/rustbot.rpm

# Install RPM and remove install file 
RUN rpm -ivh rustbot.rpm; \
    rm rustbot.rpm

# Need this environment variable to tell rustbot it's inside a container
# Not all features are supported within the container
ENV IS_RUNNING_IN_CONTAINER="true"

# Entrypoint for rustbot
CMD /opt/rustbot/rustbot