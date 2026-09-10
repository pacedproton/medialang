# EVENT Examples for MDSL

This document provides comprehensive examples of EVENT usage in the MediaLanguage DSL, covering various real-world scenarios in media outlet networks.

## Table of Contents

1. [Basic Acquisition Events](#basic-acquisition-events)
2. [Partial Acquisitions and Investments](#partial-acquisitions-and-investments)  
3. [Mergers and Joint Ventures](#mergers-and-joint-ventures)
4. [Divestitures and Spin-offs](#divestitures-and-spin-offs)
5. [Regulatory and Legal Events](#regulatory-and-legal-events)
6. [Event-Relationship Integration](#event-relationship-integration)
7. [Complex Multi-Party Events](#complex-multi-party-events)
8. [Austrian Media Market Examples](#austrian-media-market-examples)

## Basic Acquisition Events

### Simple Full Acquisition

```mdsl
EVENT styria_acquires_kleine_2019 {
    type = "acquisition";
    date = "2019-06-15";
    status = "completed";
    
    entities = {
        styria_media = {
            id = 200001;
            role = "acquirer";
            stake_before = 0;
            stake_after = 100;
        };
        kleine_zeitung = {
            id = 300014;
            role = "target";
            stake_before = 100;
            stake_after = 0;
        };
    };
    
    impact = {
        transaction_value = 75000000;
        currency = "EUR";
        market_share_change = 15.2;
        employee_count = 450;
    };
    
    metadata = {
        regulatory_approval = "Austrian_Media_Authority";
        announcement_date = "2019-05-01";
        completion_date = "2019-06-15";
        advisor_buyer = "Goldman_Sachs";
        advisor_seller = "JP_Morgan";
    };
    
    @source = "Financial_Times_2019_06_16"
    @confidence = "high"
    @verified_by = "Austrian_Companies_Register"
}
```

### Cross-Border Acquisition

```mdsl
EVENT axel_springer_acquires_politico_2021 {
    type = "acquisition";
    date = "2021-08-26";
    status = "completed";
    
    entities = {
        axel_springer = {
            id = 100001;
            role = "acquirer";
            stake_after = 100;
        };
        politico = {
            id = 800001;
            role = "target";
            stake_before = 100;
        };
    };
    
    impact = {
        transaction_value = 1000000000;
        currency = "USD";
        geographic_expansion = "US_market";
        digital_transformation = "high_priority";
    };
    
    metadata = {
        strategic_rationale = "digital_news_expansion";
        regulatory_approval = "US_FTC_approved";
        eu_approval = "not_required";
    };
    
    @source = "Wall_Street_Journal_2021_08_27"
    @impact_analysis = "significant_european_us_media_consolidation"
}
```

## Partial Acquisitions and Investments

### Strategic Investment Round

```mdsl
EVENT ringier_investment_kurier_2020 {
    type = "strategic_investment";
    date = "2020-03-10";
    status = "completed";
    
    entities = {
        ringier_ag = {
            id = 400001;
            role = "investor";
            stake_before = 0;
            stake_after = 25;
        };
        kurier = {
            id = 500023;
            role = "investee";
            stake_before = 100;
            stake_after = 75;
        };
        existing_owners = {
            id = 500024;
            role = "existing_shareholder";
            stake_before = 100;
            stake_after = 75;
        };
    };
    
    impact = {
        investment_amount = 15000000;
        currency = "EUR";
        strategic_focus = "digital_expansion";
        technology_transfer = "yes";
        editorial_independence = "maintained";
    };
    
    metadata = {
        investment_type = "minority_stake";
        board_seats = 2;
        veto_rights = "strategic_decisions";
        exit_clause = "2025_put_option";
    };
    
    @source = "Der_Standard_2020_03_11"
    @category = "minority_investment"
}
```

### Venture Capital Round

```mdsl
EVENT vc_investment_digital_outlet_2022 {
    type = "venture_capital";
    date = "2022-11-15";
    status = "completed";
    
    entities = {
        lead_investor = {
            id = 900001;
            role = "lead_vc";
            stake_after = 20;
        };
        co_investor_1 = {
            id = 900002;
            role = "co_investor";
            stake_after = 15;
        };
        startup_outlet = {
            id = 700001;
            role = "startup";
            stake_before = 100;
            stake_after = 65;
        };
    };
    
    impact = {
        funding_round = "Series_A";
        amount_raised = 8000000;
        currency = "EUR";
        valuation_pre = 25000000;
        valuation_post = 33000000;
        use_of_funds = "technology_development,content_expansion";
    };
    
    metadata = {
        funding_round_number = 1;
        previous_funding = "seed_2021";
        lead_vc_fund = "Media_Tech_Ventures";
        liquidation_preference = "1x_non_participating";
    };
    
    @source = "TechCrunch_2022_11_16"
    @round_type = "growth_capital"
}
```

## Mergers and Joint Ventures

### Equal Merger

```mdsl
EVENT regional_papers_merger_2023 {
    type = "merger_of_equals";
    date = "2023-04-01";
    status = "pending_approval";
    
    entities = {
        regional_group_a = {
            id = 600001;
            role = "merging_party";
            stake_before = 100;
            stake_after = 50;
        };
        regional_group_b = {
            id = 600002;
            role = "merging_party";
            stake_before = 100;
            stake_after = 50;
        };
        merged_entity = {
            id = 600003;
            role = "resulting_entity";
            stake_before = 0;
            stake_after = 100;
        };
    };
    
    impact = {
        combined_circulation = 450000;
        combined_revenue = 85000000;
        currency = "EUR";
        expected_synergies = 12000000;
        job_reductions = 120;
        market_coverage = "expanded_regional";
    };
    
    metadata = {
        merger_structure = "stock_for_stock";
        exchange_ratio = "1_to_1";
        combined_entity_name = "United_Regional_Media";
        headquarters = "Vienna";
        regulatory_review = "pending";
        expected_completion = "2023_Q4";
    };
    
    @source = "MediaWatch_Austria_2023_04_02"
    @regulatory_risk = "medium"
    @synergy_confidence = "high"
}
```

### Joint Venture Formation

```mdsl
EVENT digital_platform_jv_2022 {
    type = "joint_venture";
    date = "2022-09-15";
    status = "completed";
    
    entities = {
        media_house_1 = {
            id = 150001;
            role = "jv_partner";
            stake_before = 0;
            stake_after = 60;
        };
        tech_company = {
            id = 950001;
            role = "jv_partner";
            stake_before = 0;
            stake_after = 40;
        };
        jv_entity = {
            id = 155001;
            role = "joint_venture";
            stake_before = 0;
            stake_after = 100;
        };
    };
    
    impact = {
        initial_investment = 20000000;
        currency = "EUR";
        projected_revenue_y3 = 45000000;
        market_focus = "digital_advertising_platform";
        technology_contribution = "ai_content_optimization";
    };
    
    metadata = {
        jv_structure = "50_50_governance";
        management_control = "shared";
        technology_ip = "shared_development";
        exit_mechanism = "drag_along_tag_along";
        duration = "10_years_renewable";
    };
    
    @source = "Horizont_2022_09_16"
    @strategic_focus = "digital_transformation"
}
```

## Divestitures and Spin-offs

### Asset Divestiture

```mdsl
EVENT conglomerate_divests_radio_2023 {
    type = "divestiture";
    date = "2023-02-28";
    status = "completed";
    
    entities = {
        media_conglomerate = {
            id = 300001;
            role = "seller";
            stake_before = 100;
            stake_after = 0;
        };
        radio_specialist = {
            id = 450001;
            role = "buyer";
            stake_before = 0;
            stake_after = 100;
        };
        radio_division = {
            id = 300012;
            role = "divested_asset";
            stake_before = 100;
            stake_after = 0;
        };
    };
    
    impact = {
        transaction_value = 35000000;
        currency = "EUR";
        strategic_focus = "core_business_concentration";
        radio_stations_count = 8;
        listener_base = 2500000;
    };
    
    metadata = {
        divestiture_reason = "portfolio_optimization";
        seller_focus = "digital_platforms";
        buyer_rationale = "market_consolidation";
        employee_transfer = 180;
        brand_retention = "yes";
    };
    
    @source = "Media_Business_Weekly_2023_03_01"
    @transaction_type = "strategic_divestiture"
}
```

### Corporate Spin-off

```mdsl
EVENT digital_unit_spinoff_2023 {
    type = "spinoff";
    date = "2023-07-01";
    status = "completed";
    
    entities = {
        parent_company = {
            id = 250001;
            role = "parent";
            stake_before = 100;
            stake_after = 51;
        };
        spun_off_entity = {
            id = 251001;
            role = "spinoff";
            stake_before = 0;
            stake_after = 100;
        };
        public_shareholders = {
            id = 999999;
            role = "new_shareholders";
            stake_before = 0;
            stake_after = 49;
        };
    };
    
    impact = {
        ipo_valuation = 120000000;
        currency = "EUR";
        shares_issued = 10000000;
        ipo_proceeds = 58800000;
        market_focus = "digital_first_media";
    };
    
    metadata = {
        spinoff_method = "ipo";
        listing_exchange = "Vienna_Stock_Exchange";
        parent_retention = "controlling_stake";
        management_independence = "full";
        brand_separation = "yes";
        cross_services = "shared_technology_platform";
    };
    
    @source = "Boerse_Express_2023_07_02"
    @listing_success = "oversubscribed_2x"
}
```

## Regulatory and Legal Events

### License Transfer

```mdsl
EVENT broadcasting_license_transfer_2022 {
    type = "license_transfer";
    date = "2022-12-15";
    status = "approved";
    
    entities = {
        license_seller = {
            id = 800001;
            role = "current_licensee";
            stake_before = 100;
            stake_after = 0;
        };
        license_buyer = {
            id = 800002;
            role = "new_licensee";
            stake_before = 0;
            stake_after = 100;
        };
    };
    
    impact = {
        license_value = 25000000;
        currency = "EUR";
        coverage_area = "Vienna_metropolitan";
        frequency_spectrum = "FM_95.2";
        license_duration = "10_years";
    };
    
    metadata = {
        regulatory_authority = "Austrian_Communications_Authority";
        approval_process = "6_months";
        public_consultation = "completed";
        compliance_requirements = "local_content_40_percent";
        transfer_conditions = "employment_guarantees_2_years";
    };
    
    @source = "RTR_Official_Gazette_2022_12_16"
    @regulatory_category = "broadcasting_license"
}
```

### Regulatory Investigation Resolution

```mdsl
EVENT competition_settlement_2023 {
    type = "regulatory_settlement";
    date = "2023-05-10";
    status = "completed";
    
    entities = {
        investigated_company = {
            id = 120001;
            role = "respondent";
            stake_before = 100;
            stake_after = 85;
        };
        regulatory_authority = {
            id = 999998;
            role = "regulator";
            stake_before = 0;
            stake_after = 0;
        };
        divested_assets = {
            id = 120015;
            role = "mandated_divestiture";
            stake_before = 100;
            stake_after = 0;
        };
    };
    
    impact = {
        settlement_amount = 15000000;
        currency = "EUR";
        market_share_reduction = 15;
        competitive_remedy = "asset_divestiture";
        market_concentration_post = "reduced";
    };
    
    metadata = {
        investigation_duration = "18_months";
        competition_concern = "market_dominance";
        settlement_terms = "behavioral_and_structural";
        monitoring_period = "5_years";
        compliance_officer = "external_appointment";
    };
    
    @source = "Austrian_Competition_Authority_2023_05_11"
    @case_number = "BWB-2021-0156"
}
```

## Event-Relationship Integration

### Complete Acquisition with Relationship Changes

```mdsl
EVENT complete_acquisition_example_2023 {
    type = "acquisition";
    date = "2023-08-15";
    status = "completed";
    
    entities = {
        acquirer = {
            id = 100001;
            role = "acquirer";
            stake_after = 100;
        };
        target = {
            id = 200001;
            role = "target";
            stake_before = 100;
        };
    };
    
    impact = {
        transaction_value = 50000000;
        currency = "EUR";
    };
}

// Diachronic relationship showing ownership change
DIACHRONIC_LINK ownership_transfer_2023 {
    predecessor = 200000;  // old ownership structure
    successor = 200001;    // new ownership under acquirer
    event_date = "2023-08-15";
    relationship_type = "ownership_change";
    triggered_by_event = complete_acquisition_example_2023;
}

// New synchronous relationship created
SYNCHRONOUS_LINK parent_subsidiary_2023 {
    outlet_1 = { id = 100001; role = "parent"; };
    outlet_2 = { id = 200001; role = "subsidiary"; };
    relationship_type = "ownership";
    created_by_event = complete_acquisition_example_2023;
}

// Family structure reflecting new ownership
FAMILY "Enlarged Media Group" {
    OUTLET "Parent Company" {
        IDENTITY {
            id = 100001;
            title = "Parent Media Corp";
        }
    }
    
    OUTLET "Acquired Outlet" {
        IDENTITY {
            id = 200001;
            title = "Target Media House";
        }
    }
}
```

## Complex Multi-Party Events

### Three-Way Merger

```mdsl
EVENT regional_consolidation_merger_2024 {
    type = "three_way_merger";
    date = "2024-01-15";
    status = "pending_approval";
    
    entities = {
        company_a = {
            id = 701001;
            role = "merging_party";
            stake_before = 100;
            stake_after = 40;
        };
        company_b = {
            id = 701002;
            role = "merging_party";
            stake_before = 100;
            stake_after = 35;
        };
        company_c = {
            id = 701003;
            role = "merging_party";
            stake_before = 100;
            stake_after = 25;
        };
        merged_entity = {
            id = 701000;
            role = "resulting_entity";
            stake_before = 0;
            stake_after = 100;
        };
    };
    
    impact = {
        combined_valuation = 200000000;
        currency = "EUR";
        market_coverage = "tri_state_region";
        circulation_combined = 800000;
        digital_subscribers = 150000;
        synergy_potential = 25000000;
    };
    
    metadata = {
        merger_structure = "triangular_merger";
        exchange_ratios = "based_on_relative_valuations";
        governance = "proportional_board_representation";
        management_structure = "co_ceo_model_first_year";
        headquarters_location = "largest_market_company_a";
        brand_strategy = "maintain_regional_brands";
    };
    
    @source = "European_Media_Quarterly_2024_01_16"
    @complexity = "high"
    @regulatory_risk = "significant"
}
```

### Multi-Round Investment with Multiple Investors

```mdsl
EVENT series_b_funding_2023 {
    type = "venture_funding";
    date = "2023-10-20";
    status = "completed";
    
    entities = {
        startup_outlet = {
            id = 901001;
            role = "startup";
            stake_before = 70;
            stake_after = 45;
        };
        lead_investor = {
            id = 902001;
            role = "lead_investor";
            stake_after = 25;
        };
        strategic_investor = {
            id = 902002;
            role = "strategic_investor";
            stake_after = 15;
        };
        follow_on_investor = {
            id = 902003;
            role = "follow_on_investor";
            stake_after = 10;
        };
        employee_pool = {
            id = 999997;
            role = "employee_options";
            stake_after = 5;
        };
    };
    
    impact = {
        funding_round = "Series_B";
        amount_raised = 25000000;
        currency = "EUR";
        pre_money_valuation = 75000000;
        post_money_valuation = 100000000;
        runway_months = 36;
    };
    
    metadata = {
        use_of_funds = "50_percent_technology,30_percent_content,20_percent_marketing";
        board_composition = "2_investor_seats,2_founder_seats,1_independent";
        liquidation_preference = "1x_participating_preferred";
        anti_dilution = "weighted_average";
        drag_along_threshold = "majority_preferred";
    };
    
    @source = "VentureWire_Europe_2023_10_21"
    @funding_quality = "high_tier_investors"
}
```

## Austrian Media Market Examples

### ORF Restructuring Event

```mdsl
EVENT orf_digital_transformation_2023 {
    type = "organizational_restructuring";
    date = "2023-06-01";
    status = "implemented";
    
    entities = {
        orf_main = {
            id = 1001;
            role = "parent_organization";
            stake_before = 100;
            stake_after = 100;
        };
        orf_digital = {
            id = 1002;
            role = "new_digital_division";
            stake_before = 0;
            stake_after = 100;
        };
    };
    
    impact = {
        budget_allocation = 50000000;
        currency = "EUR";
        strategic_focus = "streaming_services";
        employee_transfer = 200;
        new_platform_launches = 3;
    };
    
    metadata = {
        restructuring_type = "internal_reorganization";
        regulatory_approval = "ORF_Law_Amendment_2023";
        digital_strategy = "compete_with_international_platforms";
        public_mission = "maintained";
        commercial_activities = "expanded_digital_only";
    };
    
    @source = "ORF_Annual_Report_2023"
    @public_media_transformation = "significant"
}
```

### Kronen Zeitung Ownership Change

```mdsl
EVENT kronen_zeitung_succession_2024 {
    type = "ownership_succession";
    date = "2024-03-01";
    status = "planned";
    
    entities = {
        founding_family = {
            id = 501001;
            role = "current_owner";
            stake_before = 100;
            stake_after = 70;
        };
        management_buyout = {
            id = 501002;
            role = "mbo_team";
            stake_before = 0;
            stake_after = 20;
        };
        strategic_partner = {
            id = 501003;
            role = "media_investor";
            stake_before = 0;
            stake_after = 10;
        };
    };
    
    impact = {
        valuation = 300000000;
        currency = "EUR";
        editorial_independence = "guaranteed";
        circulation_impact = "minimal_expected";
        employment_security = "5_year_guarantee";
    };
    
    metadata = {
        succession_planning = "5_year_process";
        editorial_charter = "unchanged";
        market_position = "maintain_leadership";
        digital_investment = "accelerated";
        family_involvement = "advisory_board";
    };
    
    @source = "Wirtschaftsblatt_2024_03_02"
    @market_significance = "extremely_high"
    @editorial_impact = "monitored_closely"
}
```

## Usage Patterns

### Event Sequences

```mdsl
// Initial investment
EVENT initial_investment_2022 {
    type = "seed_funding";
    date = "2022-06-01";
    // ... details
}

// Follow-up acquisition triggered by success
EVENT follow_up_acquisition_2024 {
    type = "acquisition";
    date = "2024-01-15";
    
    entities = {
        original_investor = {
            id = 800001;
            role = "acquirer";
            stake_after = 100;
        };
        startup = {
            id = 800002;
            role = "target";
            stake_before = 100;
        };
    };
    
    metadata = {
        preceded_by = "initial_investment_2022";
        investment_success = "exceeded_projections";
    };
}
```

### Event Annotations for Analytics

```mdsl
EVENT market_consolidation_2023 {
    type = "acquisition";
    date = "2023-11-01";
    // ... standard fields
    
    @market_trend = "consolidation"
    @geographic_impact = "regional"
    @strategic_significance = "high"
    @financial_model = "synergy_driven"
    @regulatory_concern = "medium"
    @innovation_impact = "technology_integration"
    @employment_effect = "net_positive"
    @competitive_response = "defensive_positioning"
}
```

These examples demonstrate the flexibility and power of the EVENT construct for modeling complex temporal relationships in media outlet networks, providing rich data for analysis and understanding of market dynamics.