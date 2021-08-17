gcloud beta container \
--project "monadium" \
clusters create-auto "monadium-cluster" \
--region "europe-north1" \
--release-channel "regular" \
--network "projects/monadium/global/networks/default" \
--subnetwork "projects/monadium/regions/europe-north1/subnetworks/default" \
--cluster-ipv4-cidr "/17" \
--services-ipv4-cidr "/22"
