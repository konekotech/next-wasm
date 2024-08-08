import { createRequire } from 'module';
const require = createRequire(import.meta.url);

export default {
    webpack: (config, { isServer }) => {
        config.experiments = {
            asyncWebAssembly: true,  // または syncWebAssembly: true
            layers: true,            // 必要に応じて layers を有効にします
        };

        config.module.rules.push({
            test: /\.wasm$/,
            type: 'webassembly/async',  // または 'webassembly/sync'
        });

        return config;
    },
};
